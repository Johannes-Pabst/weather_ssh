use rand::{Rng, rngs::ThreadRng};

const VERBS: &[&str] = &[
    "Enter",
    "Acess",
    "Align",
    "Build",
    "Calibrat",
    "Instanc",
    "Configur",
    "Tweak",
    "Hack",
    "Pwn",
    "Boot",
    "Allocat",
    "Bind",
    "Revv",
    "Polish",
    "Fabricat",
    "Ping",
    "Refactor",
    "Load",
    "Quantify",
    "Assembl",
    "Distill",
    "Bak",
    "Receiv",
    "Unlock",
    "Compil",
    "Chooch",
    "Mak",
    "Engag",
    "Decrypt",
    "Synthesiz",
    "Predict",
    "Analyz",
    "Dispens",
    "Insert",
    "Align",
    "Encourag",
    "Extrud",
    "Access",
    "Sharpen",
    "Enhanc",
    "Crank",
    "Stack",
    "Craft",
    "Render",
    "Mount",
    "Generat",
    "Implement",
    "Download",
    "Construct",
    "Customiz",
    "Compensat",
    "Buffer",
    "Transferr",
    "Induct",
    "Emitt",
    "Unzipp",
    "Spark",
    "Implant",
    "Triangulat",
    "Inject",
    "Link",
    "Brew",
    "Process",
    "Deploy",
    "Tun",
    "Attach",
    "Train",
    "Ignor",
    "Reload",
    "Simulat",
    "Fill",
    "Sort",
    "Updat",
    "Upgrad",
    "Prim",
    "Trac",
    "Inflat",
    "Charg",
    "Crack",
    "Ignor",
    "Activat",
    "Collect",
    "Approv",
    "Sampl",
    "Energiz",
    "Stuff",
    "Sustain",
    "Decrypt",
    "Reconfigur",
    "Install",
    "Secur",
    "Validat",
    "Insert",
    "Repair",
    "Support",
    "Sandboxing",
];
const UNVERBS: &[&str] = &[
    "Deallocat",
    "Trash",
    "Unplugg",
    "Revok",
    "Forgett",
    "Discard",
    "Dropp",
    "Releas",
    "Collimat",
    "Eject",
    "Ditch",
    "Leak",
    "Dereferenc",
    "Destruct",
    "Decompil",
    "Blow",
    "Disengag",
    "Digest",
    "Encrypt",
    "Crash",
    "Lock",
    "Purg",
    "Rewind",
    "Free",
    "Delet",
    "Clos",
    "Collaps",
    "Stow",
    "Archiv",
    "Suspend",
    "Suppress",
    "Clean",
    "Secur",
    "Dump",
    "Obfuscat",
    "Break",
    "Scrubb",
    "Abandon",
    "Flatten",
    "Stash",
    "Finish",
    "Evacuat",
    "Scrambl",
    "Recycl",
    "Crush",
    "Zipp",
    "Unload",
    "Disconnect",
    "Loosen",
    "Contain",
    "Detach",
    "Neutraliz",
    "Salvag",
    "Empty",
    "Hid",
    "Disarm",
    "Pickl",
    "Disregard",
    "Scrapp",
    "Deflat",
    "Discharg",
    "Deactivat",
    "Steriliz",
    "Reliev",
    "Nuk",
    "Degauss",
    "Dismiss",
    "Drain",
    "Reject",
    "Nerf",
    "Pay",
    "Return",
    "Unstick",
    "Splitt",
    "Cancell",
    "Sham",
    "Embezzl",
    "Fling",
    "Regrett",
    "Halt",
    "Arrest",
    "Bury",
    "Unplug",
    "Destroy",
    "Demolish",
    "Encrypt",
    "Uninstall",
    "Invalidat",
    "Remove",
    "Reformat",
    "Kill",
    "Downgrade",
    "Overload",
    "Overclock",
    "Underclock",
    "Misus",
];
const FUNNOUNS: &[&str] = &[
    "radiation",
    "malware",
    "trojan",
    "password stealer",
    "virus",
    "linux",
    "spy",
    "camera",
    "cmd",
    "backdoor",
    "spam",
    "adware",
    "bloatware",
    "terminal",
    "obfuscation",
    "botnet",
    "the dark web",
    "executable",
    "nuke",
    "trap",
    "lightning",
];
const UNNOUNS: &[&str] = &[
    "bugs",
    "loud noises",
    "lasers",
    "fedora",
    "IP address",
    "explosives",
    "crypto miners",
    "DDOS attacks",
    "phishing links",
    "rats",
    "coders",
    "logarithms",
];
const FNOUNS: &[&str] = &[
    "browser",
    "content",
    "API",
    "warp drive",
    "data",
    "AI",
    "bytecode",
    "signal",
    "password",
    "privacy",
    "synergy",
    "reality",
    "voltage",
    "the core",
    "steam",
    "protocol",
    "software",
    "the future",
    "5G implant",
    "the Internet",
    "neural net",
    "paperwork",
    "kernel",
    "algorithm",
    "licence",
    "loading screen",
    "debugger",
    "cache",
    "hard drive",
    "RAM",
    "keyboard",
    "mouse",
    "graphics card",
    "CPU",
    "motherboard",
    "SSD",
    "system 32",
    "system clock",
    "bootloader",
    "BIOS",
    "UEFI",
    "support",
    "memory",
    "evidence",
];
const NOUNS: &[&str] = &[
    "peripherals",
    "packages",
    "username",
    "mainframe",
    "measurements",
    "electrons",
    "wires",
    "bits",
    "sensors",
    "photons",
    "chips",
    "circuits",
    "widgets",
    "packets",
    "protocols",
    "registers",
    "subroutines",
    "holograms",
    "magnets",
    "inductors",
    "resistors",
    "capacitors",
    "vectors",
    "fluids",
    "comments",
    "ports",
    "variables",
    "antivirus",
    "windows",
    "macros",
    "pointers",
    "personal photos",
    "drivers",
    "matrices",
];
const UADJECTIVES: &[&str] = &[
    "third-party",
    "vulnerable",
    "unofficial",
    "quarantized",
    "untrusted",
    "secret",
    "wrong",
    "suspicious",
    "rejected",
    "harmful",
    "obfuscated",
    "unencrypted",
    "polluted",
    "classified",
    "invasive",
    "python",
    "legacy",
    "unsupported",
    "unknown",
    "misleading",
    "vibecoded",
    "niche",
    "orphaned",
];
const ADJECTIVES: &[&str] = &[
    "binary",
    "special",
    "specific",
    "supported",
    "mega",
    "super",
    "static",
    "immutable",
    "known",
    "excited",
    "marked",
    "undefined",
    "random",
    "unused",
    "custom",
    "harmless",
    "secure",
    "uncommon",
    "stranded",
];
const ERRORTEMPLATES: &[&str] = &[
    "missing {snoun}",
    "unrecognized {snoun}",
    "unsupported {snoun}",
    "no {snoun} found",
    "{gnoun} {unverb}ed",
];
fn random(rng: &mut ThreadRng, list: &[&str]) -> String {
    list[rng.random_range(0..list.len())].to_string()
}
pub fn generate_message() -> String {
    let positive = false;
    let mut rng = rand::rng();
    let unverb = rng.random_range(0.0..1.0) < 0.5;
    let verb = random(
        &mut rng,
        match unverb {
            true => UNVERBS,
            false => VERBS,
        },
    );
    let noun = generate_noun(&mut rng, unverb == positive);
    format!("{verb}ing {noun}")
}
pub fn generate_error() -> String {
    let mut rng = rand::rng();
    let mut template = random(&mut rng, ERRORTEMPLATES);
    while template.contains("{snoun}") {
        template = template.replacen("{snoun}", &generate_snoun(&mut rng, false).0, 1);
    }
    while template.contains("{gnoun}") {
        template = template.replacen("{gnoun}", &generate_noun(&mut rng, false), 1);
    }
    while template.contains("{unverb}") {
        template = template.replacen("{unverb}", &random(&mut rng, UNVERBS), 1);
    }
    template
}
/*
unoun: unooun | unoun noun | noun unoun
noun: noun | noun noun
unoun: unoun | uadjective noun | adjective unoun
noun: noun | adjective noun
negative: uverb noun | verb unoun
positive: verb noun | uverb unoun
*/
pub fn generate_noun(rng: &mut ThreadRng, u: bool) -> String {
    let mut the = false;
    let noun = if u {
        let adj: f32 = rng.random();
        match adj {
            ..0.5 => extract_the(&mut the, generate_snoun(rng, true)),
            0.5..0.75 => format!(
                "{} {}",
                random(rng, UADJECTIVES),
                extract_the(&mut the, generate_snoun(rng, false))
            ),
            0.75.. => format!(
                "{} {}",
                random(rng, ADJECTIVES),
                extract_the(&mut the, generate_snoun(rng, true))
            ),
            a => panic!("{a}"),
        }
    } else {
        let snoun = extract_the(&mut the, generate_snoun(rng, false));
        let adj: f32 = rng.random();
        match adj {
            ..0.5 => snoun,
            0.5.. => format!("{} {snoun}", random(rng, ADJECTIVES)),
            a => panic!("{a}"),
        }
    };
    let noun = noun
        .split_whitespace()
        .filter(|w| *w != "the")
        .collect::<Vec<_>>()
        .join(" ");
    if the { format!("the {noun}") } else { noun }
}

fn generate_snoun(rng: &mut ThreadRng, u: bool) -> (String, bool) {
    let kind: f32 = rng.random();
    let nouns = &[NOUNS, FNOUNS].concat()[..];
    let unnouns = &[UNNOUNS, FUNNOUNS].concat()[..];
    let mut the = false;
    let noun = if u {
        match kind {
            ..0.5 => random(rng, unnouns),
            0.5..0.75 => format!(
                "{} {}",
                random(rng, FUNNOUNS),
                check_the(&mut the, random(rng, nouns))
            ),
            0.75.. => format!(
                "{} {}",
                random(rng, FNOUNS),
                check_the(&mut the, random(rng, unnouns))
            ),
            a => panic!("{a}"),
        }
    } else {
        match kind {
            ..0.5 => random(rng, nouns),
            0.5.. => format!(
                "{} {}",
                random(rng, FNOUNS),
                check_the(&mut the, random(rng, nouns))
            ),
            a => panic!("{a}"),
        }
    };
    (noun, the)
}
fn check_the(the: &mut bool, noun: String) -> String {
    let words = noun.split_whitespace().collect::<Vec<&str>>();
    *the |= words.contains(&"the");
    noun
}
fn extract_the(the: &mut bool, touple: (String, bool)) -> String {
    *the |= touple.1;
    touple.0
}
