use std::fmt::format;
use std::io;
use reqwest;
use serde_json::{Result, Value};

fn main() {
    let countries: Vec<String> = vec!["afghanistan", "albania", "algeria", "angola", "anguilla", "antiguaandbarbuda", "argentina", "armenia", "aruba", "australia", "austria", "azerbaijan", "bahamas", "bahrain", "bangladesh", "barbados", "belarus", "belgium", "belize", "benin", "bhutane", "bih", "bolivia", "botswana", "brazil", "bulgaria", "burkinafaso", "burundi", "cambodia", "cameroon", "canada", "capeverde", "caymanislands", "chad", "chile", "china", "colombia", "comoros", "congo", "costarica", "croatia", "cyprus", "czech", "djibouti", "dominica", "dominicana", "easttimor", "ecuador", "egypt", "england", "equatorialguinea", "eritrea", "estonia", "ethiopia", "finland", "france", "frenchguiana", "gabon", "gambia", "georgia", "germany", "ghana", "greece", "grenada", "guadeloupe", "guatemala", "guinea", "guineabissau", "guyana", "haiti", "honduras", "hongkong", "hungary", "india", "indonesia", "ireland", "israel", "italy", "ivorycoast", "jamaica", "japan", "jordan", "kazakhstan", "kenya", "kuwait", "kyrgyzstan", "laos", "latvia", "lesotho", "liberia", "lithuania", "luxembourg", "macau", "madagascar", "malawi", "malaysia", "maldives", "mauritania", "mauritius", "mexico", "moldova", "mongolia", "montenegro", "montserrat", "morocco", "mozambique", "myanmar", "namibia", "nepal", "netherlands", "newcaledonia", "newzealand", "nicaragua", "niger", "nigeria", "northmacedonia", "norway", "oman", "pakistan", "panama", "papuanewguinea", "paraguay", "peru", "philippines", "poland", "portugal", "puertorico", "reunion", "romania", "russia", "rwanda", "saintkittsandnevis", "saintlucia", "saintvincentandgrenadines", "salvador", "samoa", "saotomeandprincipe", "saudiarabia", "senegal", "serbia", "seychelles", "sierraleone", "singapore", "slovakia", "slovenia", "solomonislands", "southafrica", "spain", "srilanka", "suriname", "swaziland", "sweden", "switzerland", "taiwan", "tajikistan", "tanzania", "thailand", "tit", "togo", "tonga", "tunisia", "turkey", "turkmenistan", "turksandcaicos", "uganda", "ukraine", "uruguay", "usa", "uzbekistan", "venezuela", "vietnam", "virginislands", "zambia", "zimbabwe"].into_iter().map(|a| a.to_string()).collect();
    let products: Vec<String> = vec!["1688", "23red", "32red", "99app", "ace2three", "adidas", "agroinform", "airbnb", "airtel", "aitu", "akelni", "alfa", "algida", "alibaba", "aliexpress", "alipay", "amasia", "amazon", "aol", "apple", "astropay", "auchan", "avito", "avon", "azino", "b4ucabs", "baidu", "banqi", "bigolive", "billmill", "bisu", "bitaqaty", "bitclout", "bittube", "blablacar", "blizzard", "blockchain", "blued", "bolt", "brand20ua", "burgerking", "bykea", "cafebazaar", "caixa", "careem", "carousell", "cdkeys", "cekkazan", "citaprevia", "citymobil", "clickentregas", "cliqq", "clubhouse", "cmtcuzdan", "coinbase", "coinfield", "craigslist", "cryptocom", "dbrua", "deliveroo", "delivery", "dent", "dhani", "didi", "digikala", "discord", "disneyhotstar", "divar", "dixy", "dodopizza", "domdara", "dominospizza", "dostavista", "douyu", "dream11", "drom", "drugvokrug", "dukascopy", "easypay", "ebay", "ebikegewinnspiel", "edgeless", "electroneum", "eneba", "ezbuy", "faberlic", "facebook", "fiqsy", "fiverr", "foodpanda", "foody", "forwarding", "freecharge", "galaxy", "gamearena", "gameflip", "gamekit", "gamer", "gcash", "get", "getir", "gett", "gg", "gittigidiyor", "global24", "globaltel", "globus", "glovo", "google", "grabtaxi", "green", "grindr", "hamrahaval", "happn", "haraj", "hepsiburadacom", "hezzl", "hily", "hopi", "hqtrivia", "humblebundle", "humta", "huya", "icard", "icq", "icrypex", "ifood", "immowelt", "imo", "inboxlv", "indriver", "ininal", "instagram", "iost", "iqos", "irancell", "ivi", "iyc", "jd", "jkf", "justdating", "justdial", "kakaotalk", "karusel", "keybase", "komandacard", "kotak811", "kucoinplay", "kufarby", "kvartplata", "kwai", "lazada", "lbry", "lenta", "lianxin", "line", "linkedin", "livescore", "magnit", "magnolia", "mailru", "mamba", "mcdonalds", "meetme", "mega", "mercado", "michat", "microsoft", "miloan", "miratorg", "mobile01", "momo", "monese", "monobank", "mosru", "mrgreen", "mtscashback", "myfishka", "myglo", "mylove", "mymusictaste", "mzadqatar", "nana", "naver", "ncsoft", "netflix", "nhseven", "nifty", "nike", "nimses", "nrjmusicawards", "nttgame", "odnoklassniki", "offerup", "offgamers", "okcupid", "okey", "okta", "olacabs", "olx", "onlinerby", "openpoint", "oraclecloud", "oriflame", "other", "ozon", "paddypower", "pairs", "papara", "paxful", "payberry", "paycell", "paymaya", "paypal", "paysend", "paytm", "peoplecom", "perekrestok", "pgbonus", "picpay", "pof", "pokec", "pokermaster", "potato", "powerkredite", "prajmeriz2020", "premiumone", "prom", "proton", "protonmail", "protp", "pubg", "pureplatfrom", "pyaterochka", "pyromusic", "q12trivia", "qiwiwallet", "quipp", "rakuten", "rambler", "rediffmail", "reuse", "ripkord", "rosakhutor", "rsa", "rutube", "samokat", "seosprint", "sheerid", "shopee", "signal", "sikayetvar", "skout", "snapchat", "snappfood", "sneakersnstuff", "socios", "sportmaster", "spothit", "ssoidnet", "steam", "surveytime", "swvl", "taksheel", "tango", "tantan", "taobao", "telegram", "tencentqq", "ticketmaster", "tiktok", "tinder", "tosla", "totalcoin", "touchance", "trendyol", "truecaller", "twitch", "twitter", "uber", "ukrnet", "uploaded", "vernyi", "vernyj", "viber", "vitajekspress", "vkontakte", "voopee", "wechat", "weibo", "weku", "weststein", "whatsapp", "wildberries", "wingmoney", "winston", "wish", "wmaraci", "wolt", "yaay", "yahoo", "yalla", "yandex", "yemeksepeti", "youdo", "youla", "youstar", "zalo", "zoho", "zomato"].into_iter().map(|a| a.to_string()).collect();
    let mut choice = String::new();
    let mut bestpricect = String::new();
    let mut countries_tested = 0;
    let mut best_price = 1337.0;
    winconsole::console::set_title("5SIM Searcher").unwrap();
    while !products.contains(&choice) {
        choice.clear();
        eprint!("Product name: ");
        io::stdin().read_line(&mut choice).expect("Impossible to read the line :(");
        choice.pop();
        if choice.contains("\r") { choice.pop(); }
        winconsole::console::clear();
    };
    eprintln!("Searching the best price for {choice}...");
    for country in countries {
        let res = reqwest::blocking::get(format!("https://5sim.net/v1/guest/products/{}/any", country)).unwrap()
            .text()
            .unwrap();
        let value: Value = serde_json::from_str(res.trim()).unwrap();
        let ok: Value = value[&choice]["Price"].clone();
        let convert = if ok.to_string().contains("null") {
            String::from("1336.0")
        } else {
            if !ok.to_string().contains(".") {
                String::from(format!("{}.0", value[&choice]["Price"]))
            } else {
                String::from(format!("{}", value[&choice]["Price"]))
            }
        };
        let price = convert.parse::<f32>().unwrap();
        if price < best_price {
            best_price = price.clone();
            bestpricect = country.clone();
        }
        countries_tested += 1;
        winconsole::console::set_title(format!("5SIM Searcher | Countries Tested: {} | Current Country: {}", countries_tested, country).trim());
    }
    winconsole::console::clear();
    println!("{bestpricect} Best Price: {best_price}");
    io::stdin().read_line(&mut String::new()).unwrap();
}