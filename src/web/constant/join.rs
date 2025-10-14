// Join Page Video

use crate::web::model::join::{CorpCardData, FAQEntry};

pub const FEATURED_VIDEO: &str = "https://www.youtube.com/embed/AdfFnTt2UT0?si=x3rGt9pHRJHZ9g8i";

pub const AUTUMN_ORDER_CORP_INFO: CorpCardData = CorpCardData {
    name: "The Order of Autumn",
    corporation_id: 98785281,
    location: "Nullsec",
    cta_text: "Begin Your Journey in Nullsec",
};

pub const AUTUMN_INC_CORP_INFO: CorpCardData = CorpCardData {
    name: "Autumn Inc.",
    corporation_id: 98812612,
    location: "Highsec",
    cta_text: "Begin Your Journey in Highsec",
};

pub const FAQ: &[FAQEntry] = &[
    FAQEntry {
        question: "What is Autumn's main focus?",
        answer: "Our primary focus is newer players & helping them grow, outside of that we focus on PvP with a mix of industry & PvE.",
    },
    FAQEntry {
        question: "What is Autumn's end goal?",
        answer: "Our end goal is to make the new player experience in EVE more accessible and engaging. We also aim to build a tight knit community of like minded players who share in our goal of making EVE more accessible for those new to the game.",
    },
    FAQEntry {
        question: "Is PvP mandatory?",
        answer: "While we have no monthly fleet participation minimums, we do require nullsec member attendance for the occasional Call to Action fleets which involves a highly important strategic objective we needs all hands on deck to secure. This ONLY applies if you are online & in-game, don't lose sleep over CTAs, real life always comes first."
    },
    FAQEntry {
        question: "Where is Autumn located?",
        answer: "The Order of Autumn is located within our alliance Black Rose's space in the region of Delve along with our coalition, Phoenix Coalition."
    },
    FAQEntry {
        question: "What do you use for voice chat?",
        answer: "We primarily use Discord for anything corporation or alliance level. Larger scale coalition fleets use Mumble because multiple channels that can be linked together are essential for large fleets."
    },
    FAQEntry {
        question: "What are the requirements to join?",
        answer: "To join you simply need to put an application by following one of the 'Begin Your Journey' buttons. You will also need to add your character to an application called SeAT which we use for background checks to mitigate the risk of spies. Our application process will walk you through how to do all of that."
    },
    FAQEntry {
        question: "Do I need a microphone to join?",
        answer: "No, you do not need a microphone to join. However, we do recommend having one to participate in voice comms."
    },
];

pub static EVE_LEGAL_STATEMENT: &str = "EVE Online and the EVE logo are the registered trademarks of CCP hf. All rights are reserved worldwide. All other trademarks are the property of their respective owners. EVE Online, the EVE logo, EVE and all associated logos and designs are the intellectual property of CCP hf. All artwork, screenshots, characters, vehicles, storylines, world facts or other recognizable features of the intellectual property relating to these trademarks are likewise the intellectual property of CCP hf. CCP hf. has granted permission to Autumn to use EVE Online and all associated logos and designs for promotional and information purposes on its website but does not endorse, and is not in any way affiliated with, Autumn. CCP is in no way responsible for the content on or functioning of this website, nor can it be liable for any damage arising from the use of this website.";
