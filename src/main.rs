use std::{fmt, io::Write, str::{SplitWhitespace, ParseBoolError}, process::exit, collections::HashMap};
use regex::Regex;

enum ParsedInput {
    // Meta-commands
    Quit,
    Inv,
    // Actions
    Look(String),
    Get(String),
    Use(String),
    UseOn(String, String),
    Talk(String),
    // Directions
    North,
    South,
    East,
    West,
    Down,
    Up,
    // Catch-all
    Other(String)
}

impl fmt::Display for ParsedInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Meta-commands
            ParsedInput::Quit                          => write!(f, "Quit"),
            ParsedInput::Inv                           => write!(f, "Inv"),
            // Actions
            ParsedInput::Look(s)              => write!(f, "Look({})", s),
            ParsedInput::Get(s)               => write!(f, "Get({})", s),
            ParsedInput::Use(s)               => write!(f, "Use({})", s),
            ParsedInput::UseOn(s, t) => write!(f, "UseOn({}, {})", s, t),
            ParsedInput::Talk(s)              => write!(f, "Talk({})", s),
            // Directions
            ParsedInput::North                         => write!(f, "North"),
            ParsedInput::South                         => write!(f, "South"),
            ParsedInput::East                          => write!(f, "East"),
            ParsedInput::West                          => write!(f, "West"),
            ParsedInput::Down                          => write!(f, "Down"),
            ParsedInput::Up                            => write!(f, "Up"),
            // Catch-all
            ParsedInput::Other(s)             => write!(f, "Other({})", s),
        }
    }
}

struct Inventory {
    items: Vec<(String, String)>
}

struct Flags {
    flags: HashMap<String, bool>
}

impl Flags {
    fn new() -> Flags
    {
        Flags { flags: HashMap::new() }
    }
    
    fn is_set(&self, flag: &str) -> bool
    {
        *self.flags.get(flag).unwrap_or(&false)
    }

    fn set(&mut self, flag: &str) {
        self.set_as(flag, true);
    }

    fn set_as(&mut self, flag: &str, val: bool)
    {
        self.flags.insert(String::from(flag), val);
    }
}

type Room = fn(&mut Inventory, &mut Flags) -> String;

impl Inventory {
    fn new() -> Inventory {
        Inventory { items: vec![] }
    }

    fn add(&mut self, item: &str, desc: &str) {
        self.items.push((String::from(item), String::from(desc)));
    }

    fn remove(&mut self, item: &str) {
        match self.find(item) {
            Some(i) => self.items.retain(|(itm, _)| itm != item),
            None => {}
        };
    }

    fn find(&self, target_item: &str) -> Option<usize> {
        for (i, tupl) in self.items.iter().enumerate() {
            if target_item == tupl.0 {
                return Some(i);
            }
        }
        None
    }

    fn has(&self, target_item: &str) -> bool {
        self.find(target_item).is_some()
    }
}

impl fmt::Display for Inventory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--- INVENTORY ---")?;
        for (item, desc) in &self.items {
            writeln!(f, "{: <10} | {: <10}", item, desc)?;
        }
        writeln!(f, "------------------")
    }
}

fn get_user_input() -> Result<ParsedInput, ()> {
    let mut line = String::new();
    print!("> ");
    std::io::stdout().flush().expect("");
    let _ = std::io::stdin().read_line(&mut line);

    Ok::<ParsedInput, ()>(parse_input(line))
}

fn parse_input(s: String) -> ParsedInput {
    // ...this sucks to have to do.
    // This would probably just be easier with regex, but this is what I've resigned myself to, I guess.
    match s.split_whitespace().collect::<Vec<_>>().as_slice() {
        // Directions
        ["n" | "N" | "north" | "North"]   => ParsedInput::North,
        ["s" | "S" | "south" | "South"]   => ParsedInput::South,
        ["e" | "E" | "east"  | "East"]    => ParsedInput::East,
        ["w" | "W" | "west"  | "West"]    => ParsedInput::West,
        ["u" | "U" | "up"    | "Up"]      => ParsedInput::Up,
        ["d" | "D" | "down"  | "Down"]    => ParsedInput::Down,
        // Meta-commands
        ["i" | "I" | "inv"]               => ParsedInput::Inv,
        ["q" | "Q" | "quit"]  | ["Quit"]  => ParsedInput::Quit,
        // Actions
        ["get", item @ ..] | ["take", item @ ..] | ["grab", item @ ..] => ParsedInput::Get(String::from(item.join(" "))),
        ["look", "at", thing @ ..] | ["look", thing @ ..]                       => ParsedInput::Look(String::from(thing.join(" "))),
        ["talk", "to", person @ ..]  | ["talk", person @ ..]                    => ParsedInput::Talk(String::from(person.join(" "))),
        // Catch-all
        all => {
            let use_regex = Regex::new(r"^[uU]se\s+(.*)$").unwrap();
            let use_on_regex = Regex::new(r"^[uU]se\s+(.*) on (.*)$").unwrap();

            if let Some(captures) = use_on_regex.captures(&all.join(" "))
            {
                ParsedInput::UseOn(String::from(captures.get(1).map_or("", |m| m.as_str())), 
                                   String::from(captures.get(2).map_or("", |m| m.as_str())))
            }
            else if let Some(captures) = use_regex.captures(&all.join(" "))
            {
                ParsedInput::Use(String::from(captures.get(1).map_or("", |m| m.as_str())))
            }
            else
            {
                ParsedInput::Other(String::from(all.join(" ")))
            }
        }
    }
}

fn test_room(inv: &mut Inventory, flags: &mut Flags) -> String {
    println!("To the north is Room A.");

    if !flags.is_set("test_room_intro")
    {
        println!("This should only show the first time you enter test_room.");
        flags.set("test_room_intro");
    }

    String::from(match get_user_input() {
        Ok(ParsedInput::North) => "room_a",
        Ok(ParsedInput::Look(_)) => "test_room",
        Ok(ParsedInput::Inv) => { println!("{}", inv); "test_room" },
        Ok(ParsedInput::Quit) => { exit(0) },
        _ => "test_room"
    })
}

fn room_a(inv: &mut Inventory, flags: &mut Flags) -> String {
    println!("You're in Room A! That's pretty neat!");
    if inv.has("Golden Key")
    {
        inv.remove("Golden Key");
        inv.add("Sword", "You could do some real damage with this.");
        println!("You use the Golden Key from Room B to open a chest. You found a sword inside!");
    }

    String::from("room_a")
}

fn dead_room(_inv: &mut Inventory, _flags: &mut Flags) -> String
{
    println!("Attempting to access a room that doesn't exist.");
    exit(1);
}

fn main() -> std::io::Result<()> {
    let mut inv: Inventory = Inventory::new();
    let mut flags: Flags = Flags::new();

    let mut room = String::from("test_room");

    let mut rooms: HashMap<String, Room> = HashMap::new();

    rooms.insert(String::from("test_room"), test_room);
    rooms.insert(String::from("room_a"), room_a);
    
    loop {
        room = match rooms.get(&room) {
            Some(room_fn) => *room_fn,
            _ => dead_room
        }(&mut inv, &mut flags);
    }
}

