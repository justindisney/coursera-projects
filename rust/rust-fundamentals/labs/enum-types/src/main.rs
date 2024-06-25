#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    LakeCounty,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn popularity(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Not very popular"),
        WineRegions::Bordeaux => println!("Highly popular"),
        WineRegions::Tuscany => println!("Highly popular"),
        WineRegions::Champagne => println!("Highly popular"),
        WineRegions::NapaValley => println!("Highly popular"),
        WineRegions::LakeCounty => println!("Unheard of"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Vigilance"),
        region: WineRegions::LakeCounty,
    };

    println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    println!("Wine 3: {} from {:?}", wine3.name, wine3.region);
    // supported_regions(wine1.region);
    // supported_regions(WineRegions::Rioja);
    popularity(wine1.region);
    popularity(wine2.region);
    popularity(wine3.region);
}
