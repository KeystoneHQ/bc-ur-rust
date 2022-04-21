use std::ascii;
use hex::{FromHex};
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::{utils};

const WORDS: &str = "ableacidalsoapexaquaarchatomauntawayaxisbackbaldbarnbeltbetabiasbluebodybragbrewbulbbuzzcalmcashcatschefcityclawcodecolacookcostcruxcurlcuspcyandarkdatadaysdelidicedietdoordowndrawdropdrumdulldutyeacheasyechoedgeepicevenexamexiteyesfactfairfernfigsfilmfishfizzflapflewfluxfoxyfreefrogfuelfundgalagamegeargemsgiftgirlglowgoodgraygrimgurugushgyrohalfhanghardhawkheathelphighhillholyhopehornhutsicedideaidleinchinkyintoirisironitemjadejazzjoinjoltjowljudojugsjumpjunkjurykeepkenokeptkeyskickkilnkingkitekiwiknoblamblavalazyleaflegsliarlimplionlistlogoloudloveluaulucklungmainmanymathmazememomenumeowmildmintmissmonknailnavyneednewsnextnoonnotenumbobeyoboeomitonyxopenovalowlspaidpartpeckplaypluspoempoolposepuffpumapurrquadquizraceramprealredorichroadrockroofrubyruinrunsrustsafesagascarsetssilkskewslotsoapsolosongstubsurfswantacotasktaxitenttiedtimetinytoiltombtoystriptunatwinuglyundouniturgeuservastveryvetovialvibeviewvisavoidvowswallwandwarmwaspwavewaxywebswhatwhenwhizwolfworkyankyawnyellyogayurtzapszerozestzinczonezoom";
const BYTEWORD_LENGTH: i32 = 4;
const MINIMAL_BYTEWORD_LENGTH: i32 = 2;
const DIM: u32 = 26;

lazy_static! {
    static ref BYTEWORDS_LOOK_UP_TABLE: Mutex<Vec<i32>> = {
        let mut vector = vec![0; (DIM*DIM) as usize];
        for i in 0..BYTEWORDS_NUM {
            let byteword = get_word(i as usize);
            let x = byteword.chars().nth(0).unwrap() as u32 - 'a' as u32;
            let y = byteword.chars().nth(3).unwrap() as u32  - 'a' as u32;
            let offset = y * DIM + x;
            vector[offset as usize] = i
        }
        Mutex::new(vector)
    };
}

const BYTEWORDS_NUM: i32 = 256;

pub enum STYLES {
    STANDARD,
    URI,
    MINIMAL,
}

fn get_word(index: usize) -> String {
    WORDS[index * BYTEWORD_LENGTH as usize..(index * BYTEWORD_LENGTH as usize) + BYTEWORD_LENGTH as usize].to_string()
}

fn get_minimal_word(index: usize) -> String {
    let mut byteword = get_word(index);

    byteword.remove(1);
    byteword.remove(1);
    byteword
}

fn add_crc(message: String) -> Result<String, String> {
    let crc = utils::get_crc_hex(match Vec::from_hex(&message) {
        Ok(m) => { m }
        Err(e) => { return Err(e.to_string()); }
    });
    let mut result = message.clone();
    result.push_str(crc.as_str());
    Ok(result)
}

fn encode_with_separator(word: String, separator: String) -> Result<String, String> {
    let crc_appended_word = match add_crc(word) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let crc_word_buff = match Vec::from_hex(crc_appended_word) {
        Ok(m) => m,
        Err(e) => return Err(e.to_string())
    };
    let result = crc_word_buff.iter().fold::<Vec<String>, fn(Vec<String>, &u8) -> Vec<String>>(Vec::<String>::new(), |mut acc, x| {
        acc.push(get_word(x.clone() as usize));
        acc
    });
    Ok(result.join(separator.as_str()))
}

fn encode_minimal(word: String) -> Result<String, String> {
    let crc_appended_word = match add_crc(word) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let crc_word_buff = match Vec::from_hex(crc_appended_word) {
        Ok(m) => m,
        Err(e) => return Err(e.to_string())
    };
    Ok(crc_word_buff.iter().fold::<String, fn(String, &u8) -> String>(String::new(), |mut acc, x| {
        acc.push_str(get_minimal_word(x.clone() as usize).as_str());
        acc
    }))
}

fn decode_word(word: String, word_length: i32) -> Result<String, String> {
    if word_length as usize != word.len() {
        return Err("Invalid Bytewords: word.length does not match wordLength provided".to_string());
    }

    let word_lower = word.to_lowercase();
    let x = (word_lower.chars().nth(0).unwrap()) as u32 - 'a' as u32;
    let y = word_lower.chars().nth(match word_length {
        4 => 3,
        _ => 1
    }).unwrap() as u32 - 'a' as u32;

    if !(x < DIM && y < DIM) {
        return Err("Invalid Bytewords: invalid word".to_string());
    }

    let offset = y * DIM + x;
    let value = BYTEWORDS_LOOK_UP_TABLE.lock().unwrap()[offset as usize];

    if value == -1 {
        return Err("Invalid Bytewords: value not in lookup table".to_string());
    }

    if word_length == BYTEWORD_LENGTH {
        let byteword = get_word(value as usize);
        let c1 = word_lower.chars().nth(1).unwrap();
        let c2 = word_lower.chars().nth(2).unwrap();

        if c1 != byteword.chars().nth(1).unwrap() || c2 != byteword.chars().nth(2).unwrap() {
            return Err("Invalid Bytewords: invalid middle letters of word".to_string());
        }
    }
    Ok(format!("{:02x}", value))
}

fn _decode(string: String, separator: String, word_length: i32) -> Result<String, String> {
    let words = match word_length {
        4 => string.split(separator.as_str()).map(|x| x.to_string()).collect::<Vec<String>>(),
        _ => string.chars().collect::<Vec<char>>().chunks(2).map(|x| x.iter().collect::<String>()).collect::<Vec<String>>()
    };
    let decoded_string = match words.iter().map(|word|
        decode_word(word.clone(), word_length)
    ).collect::<Result<Vec<String>, String>>() {
        Ok(x) => x.join(""),
        Err(e) => return Err(e),
    };

    if decoded_string.len() < 5 {
        return Err("Invalid Bytewords: invalid decoded string length".to_string());
    }

    let decoded_buff = Vec::from_hex(decoded_string).unwrap();

    let body = &decoded_buff[0..decoded_buff.len() - 4];
    let body_checksum = &decoded_buff[decoded_buff.len() - 4..];
    let checksum = utils::get_crc_hex(body.to_vec());
    if checksum != hex::encode(body_checksum.to_vec()) {
        return Err("Invalid Checksum".to_string());
    }

    Ok(hex::encode(body.to_vec()))
}

pub fn decode(string: String, style: Option<STYLES>) -> Result<String, String> {
    let _style = style.unwrap_or(STYLES::MINIMAL);
    match _style {
        STYLES::STANDARD => _decode(string, " ".to_string(), BYTEWORD_LENGTH),
        STYLES::URI => _decode(string, "-".to_string(), BYTEWORD_LENGTH),
        STYLES::MINIMAL => _decode(string, "".to_string(), MINIMAL_BYTEWORD_LENGTH),
    }
}

pub fn encode(string: String, style: Option<STYLES>) -> Result<String, String> {
    let _style = style.unwrap_or(STYLES::MINIMAL);
    match _style {
        STYLES::STANDARD => encode_with_separator(string, " ".to_string()),
        STYLES::URI => encode_with_separator(string, "-".to_string()),
        STYLES::MINIMAL => encode_minimal(string),
    }
}

#[cfg(test)]
mod test {
    use crate::bytewords;
    use crate::bytewords::{decode, encode, STYLES};

    const HEX_INPUT: &str = "d9012ca20150c7098580125e2ab0981253468b2dbc5202d8641947da";
    const BUFFER_INPUT: [u8; 100] = [245, 215, 20, 198, 241, 235, 69, 59, 209, 205,
        165, 18, 150, 158, 116, 135, 229, 212, 19, 159,
        17, 37, 239, 240, 253, 11, 109, 191, 37, 242,
        38, 120, 223, 41, 156, 189, 242, 254, 147, 204,
        66, 163, 216, 175, 191, 72, 169, 54, 32, 60,
        144, 230, 210, 137, 184, 197, 33, 113, 88, 14,
        157, 31, 177, 46, 1, 115, 205, 69, 225, 150,
        65, 235, 58, 144, 65, 240, 133, 69, 113, 247,
        63, 53, 242, 165, 160, 144, 26, 13, 79, 237,
        133, 71, 82, 69, 254, 165, 138, 41, 85, 24];

    #[test]
    fn test_encode() {
        let hex_input = HEX_INPUT.to_string();
        let buffer_input = hex::encode(BUFFER_INPUT);
        //Standard
        assert_eq!(encode(hex_input.clone(), Some(STYLES::STANDARD)).unwrap(), "tuna acid draw oboe acid good slot axis limp lava brag holy door puff monk brag guru frog luau drop roof grim also trip idle chef fuel twin tied draw grim ramp");
        assert_eq!(encode(buffer_input.clone(), Some(STYLES::STANDARD)).unwrap(), "yank toys bulb skew when warm free fair tent swan open brag mint noon jury list view tiny brew note body data webs what zinc bald join runs data whiz days keys user diet news ruby whiz zone menu surf flew omit trip pose runs fund part even crux fern math visa tied loud redo silk curl jugs hard beta next cost puma drum acid junk swan free very mint flap warm fact math flap what limp free jugs yell fish epic whiz open numb math city belt glow wave limp fuel grim free zone open love diet gyro cats fizz holy city puff");

        //URI
        assert_eq!(encode(hex_input.clone(), Some(STYLES::URI)).unwrap(), "tuna-acid-draw-oboe-acid-good-slot-axis-limp-lava-brag-holy-door-puff-monk-brag-guru-frog-luau-drop-roof-grim-also-trip-idle-chef-fuel-twin-tied-draw-grim-ramp");
        assert_eq!(encode(buffer_input.clone(), Some(STYLES::URI)).unwrap(), "yank-toys-bulb-skew-when-warm-free-fair-tent-swan-open-brag-mint-noon-jury-list-view-tiny-brew-note-body-data-webs-what-zinc-bald-join-runs-data-whiz-days-keys-user-diet-news-ruby-whiz-zone-menu-surf-flew-omit-trip-pose-runs-fund-part-even-crux-fern-math-visa-tied-loud-redo-silk-curl-jugs-hard-beta-next-cost-puma-drum-acid-junk-swan-free-very-mint-flap-warm-fact-math-flap-what-limp-free-jugs-yell-fish-epic-whiz-open-numb-math-city-belt-glow-wave-limp-fuel-grim-free-zone-open-love-diet-gyro-cats-fizz-holy-city-puff");

        //Minimal
        assert_eq!(encode(hex_input.clone(), Some(STYLES::MINIMAL)).unwrap(), "taaddwoeadgdstaslplabghydrpfmkbggufgludprfgmaotpiecffltntddwgmrp");
        assert_eq!(encode(buffer_input.clone(), Some(STYLES::MINIMAL)).unwrap(), "yktsbbswwnwmfefrttsnonbgmtnnjyltvwtybwnebydawswtzcbdjnrsdawzdsksurdtnsrywzzemusffwottppersfdptencxfnmhvatdldroskcljshdbantctpadmadjksnfevymtfpwmftmhfpwtlpfejsylfhecwzonnbmhcybtgwwelpflgmfezeonledtgocsfzhycypf");
    }

    #[test]
    fn test_decode() {
        let hex_input = HEX_INPUT.to_string();
        let buffer_input = hex::encode(BUFFER_INPUT);

        //Standard
        assert_eq!(
            decode(
                "tuna acid draw oboe acid good slot axis limp lava brag holy door puff monk brag guru frog luau drop roof grim also trip idle chef fuel twin tied draw grim ramp".to_string(),
                Some(STYLES::STANDARD),
            ).unwrap(),
            hex_input.clone()
        );
        assert_eq!(
            decode(
                "yank toys bulb skew when warm free fair tent swan open brag mint noon jury list view tiny brew note body data webs what zinc bald join runs data whiz days keys user diet news ruby whiz zone menu surf flew omit trip pose runs fund part even crux fern math visa tied loud redo silk curl jugs hard beta next cost puma drum acid junk swan free very mint flap warm fact math flap what limp free jugs yell fish epic whiz open numb math city belt glow wave limp fuel grim free zone open love diet gyro cats fizz holy city puff".to_string(),
                Some(STYLES::STANDARD),
            ).unwrap(),
            buffer_input.clone()
        );

        //URI
        assert_eq!(
            decode(
                "tuna-acid-draw-oboe-acid-good-slot-axis-limp-lava-brag-holy-door-puff-monk-brag-guru-frog-luau-drop-roof-grim-also-trip-idle-chef-fuel-twin-tied-draw-grim-ramp".to_string(),
                Some(STYLES::URI),
            ).unwrap(),
            hex_input.clone()
        );
        assert_eq!(
            decode(
                "yank-toys-bulb-skew-when-warm-free-fair-tent-swan-open-brag-mint-noon-jury-list-view-tiny-brew-note-body-data-webs-what-zinc-bald-join-runs-data-whiz-days-keys-user-diet-news-ruby-whiz-zone-menu-surf-flew-omit-trip-pose-runs-fund-part-even-crux-fern-math-visa-tied-loud-redo-silk-curl-jugs-hard-beta-next-cost-puma-drum-acid-junk-swan-free-very-mint-flap-warm-fact-math-flap-what-limp-free-jugs-yell-fish-epic-whiz-open-numb-math-city-belt-glow-wave-limp-fuel-grim-free-zone-open-love-diet-gyro-cats-fizz-holy-city-puff".to_string(),
                Some(STYLES::URI),
            ).unwrap(),
            buffer_input.clone()
        );

        //Minimal
        assert_eq!(
            decode(
                "taaddwoeadgdstaslplabghydrpfmkbggufgludprfgmaotpiecffltntddwgmrp".to_string(),
                Some(STYLES::MINIMAL),
            ).unwrap(),
            hex_input.clone()
        );
        assert_eq!(
            decode(
                "yktsbbswwnwmfefrttsnonbgmtnnjyltvwtybwnebydawswtzcbdjnrsdawzdsksurdtnsrywzzemusffwottppersfdptencxfnmhvatdldroskcljshdbantctpadmadjksnfevymtfpwmftmhfpwtlpfejsylfhecwzonnbmhcybtgwwelpflgmfezeonledtgocsfzhycypf".to_string(),
                Some(STYLES::MINIMAL),
            ).unwrap(),
            buffer_input.clone()
        );
    }
}
