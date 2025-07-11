use std::collections::{HashMap, HashSet};
use std::io;
use std::io::ErrorKind;

pub fn verify_weapon_tierlist(tierlist: &[Vec<String>], weapons: &[String]) -> Result<(), io::Error> {
    let tier_set: HashSet<&str> = tierlist.iter().flatten().map(String::as_str).collect();
    let weapon_set: HashSet<&str> = weapons.iter().map(String::as_str).collect();

    let missing: Vec<&str> = weapon_set.difference(&tier_set).copied().collect();

    if !missing.is_empty() {
        let msg = format!("Missing weapons from tierlist:\n{}", missing.join("\n"));
        return Err(io::Error::new(ErrorKind::InvalidInput, msg));
    }

    Ok(())
}

pub fn create_legend_tierlist(data: (HashMap<String, (String, String)>, Vec<Vec<String>>)) -> Option<Vec<Vec<String>>> {
    let (legend_map,
        weapons_tierlist) = data;

    let weapons_vec: Vec<String> = legend_map.values()
        .flat_map(|(a, b)| [a, b])
        .cloned()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    if let Err(e) = verify_weapon_tierlist(&weapons_tierlist, &weapons_vec) {
        println!("{}", e);
        return None;
    }

    let tiers: HashMap<&str, usize> = weapons_tierlist
        .iter()
        .enumerate()
        .flat_map(|(tier, weapons)| weapons.iter().map(move |weapon| (weapon.as_str(), tier)))
        .collect();

    let mut l_tierlist: Vec<Vec<String>> =
        vec![Vec::new(); weapons_tierlist.len()];

    for (legend, (w1, w2)) in legend_map {
        let t1 = tiers[w1.as_str()];
        let t2 = tiers[w2.as_str()];
        let legend_tier = (t1 + t2 + 1) / 2;
        l_tierlist[legend_tier].push(legend);
    }
    
    Some(l_tierlist)
}