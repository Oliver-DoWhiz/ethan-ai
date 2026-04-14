use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize)]
pub struct TemplateDefinition {
    pub slug: &'static str,
    pub name: &'static str,
    pub duration_label: &'static str,
    pub aspect_ratio: &'static str,
    pub promise: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectResponse {
    pub creative_direction: CreativeDirection,
    pub workflow: Vec<WorkflowStep>,
    pub templates: Vec<TemplateOutput>,
    pub publish_pack: PublishPack,
    pub review_notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreativeDirection {
    pub headline: String,
    pub brand_line: String,
    pub operator_brief: String,
    pub palette: Vec<&'static str>,
    pub motion_notes: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WorkflowStep {
    pub title: &'static str,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateOutput {
    pub slug: &'static str,
    pub name: &'static str,
    pub duration_label: String,
    pub aspect_ratio: &'static str,
    pub opening_hook: String,
    pub summary: String,
    pub shot_plan: Vec<ShotPlan>,
    pub overlays: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ShotPlan {
    pub label: String,
    pub duration_label: String,
    pub purpose: String,
    pub source: String,
    pub caption: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublishPack {
    pub title_candidates: Vec<String>,
    pub description: String,
    pub chapter_suggestions: Vec<String>,
    pub thumbnail_options: Vec<String>,
    pub export_targets: Vec<ExportTarget>,
    pub subtitle_pack: SubtitlePack,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExportTarget {
    pub label: &'static str,
    pub delivery: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SubtitlePack {
    pub primary_language: &'static str,
    pub secondary_language: &'static str,
    pub styling: &'static str,
}

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub listing_title: String,
    pub city: String,
    pub neighborhood: String,
    pub price_millions: f32,
    pub beds: u8,
    pub baths: f32,
    pub sqft: u32,
    pub agent_name: String,
    pub buyer_profile: String,
    pub hero_feature: String,
    pub cta: String,
    pub brand_voice: String,
    pub hook_style: String,
    pub room_sequence: Vec<String>,
    pub assets: AssetManifest,
}

#[derive(Debug, Deserialize)]
pub struct AssetManifest {
    pub footage_clips: u16,
    pub listing_photos: u8,
    pub has_floorplan: bool,
    pub has_voice_notes: bool,
    pub has_drone: bool,
    pub has_neighborhood_broll: bool,
}

#[derive(Debug)]
pub enum GenerateError {
    MissingField(&'static str),
    MissingRooms,
    InvalidPrice,
    InvalidAssetCount(&'static str),
}

impl GenerateError {
    pub fn message(&self) -> String {
        match self {
            Self::MissingField(field) => {
                format!("`{field}` is required for Ethan to build a plan.")
            }
            Self::MissingRooms => {
                "Select at least one room or scene for the edit plan.".to_string()
            }
            Self::InvalidPrice => "Price must be greater than zero.".to_string(),
            Self::InvalidAssetCount(field) => {
                format!("`{field}` must be greater than zero for this MVP run.")
            }
        }
    }
}

const TEMPLATE_CATALOG: [TemplateDefinition; 3] = [
    TemplateDefinition {
        slug: "open-house-masterpiece",
        name: "Open House Masterpiece",
        duration_label: "4 to 6 min",
        aspect_ratio: "16:9",
        promise:
            "The flagship YouTube walkthrough with a composed arc, price framing, and a sharp CTA.",
    },
    TemplateDefinition {
        slug: "hero-short",
        name: "Hero Short",
        duration_label: "25 to 35 sec",
        aspect_ratio: "9:16",
        promise: "A vertical burst built around one irresistible feature or emotional hook.",
    },
    TemplateDefinition {
        slug: "listing-explainer",
        name: "Listing Explainer",
        duration_label: "60 to 90 sec",
        aspect_ratio: "4:5",
        promise: "Dense, factual storytelling for reposts, DMs, and channel distribution.",
    },
];

pub fn templates() -> &'static [TemplateDefinition] {
    &TEMPLATE_CATALOG
}

pub fn generate(mut input: GenerateRequest) -> Result<ProjectResponse, GenerateError> {
    trim_fields(&mut input);
    validate(&input)?;

    let price = price_label(input.price_millions);
    let home_facts = format!(
        "{} beds · {} baths · {} sq ft · {}",
        input.beds,
        format_baths(input.baths),
        input.sqft,
        price
    );
    let room_sequence = normalize_rooms(&input.room_sequence);
    let brand_line = brand_line(&input.brand_voice, &input.city);
    let hook = opening_hook(&input);
    let shot_span = longform_shot_span(room_sequence.len());

    let creative_direction = CreativeDirection {
        headline: format!("{} becomes a publish-ready story.", input.listing_title),
        brand_line,
        operator_brief: format!(
            "Lead with {}. Keep the pacing controlled, let the numbers land cleanly, and make {} feel like the natural buyer for this home.",
            input.hero_feature, input.buyer_profile
        ),
        palette: palette_for(&input.brand_voice),
        motion_notes: motion_notes(&input.brand_voice),
    };

    let workflow = vec![
        WorkflowStep {
            title: "Ingest",
            detail: format!(
                "{} clips, {} listing photos, floorplan {}, voice notes {}.",
                input.assets.footage_clips,
                input.assets.listing_photos,
                yes_no(input.assets.has_floorplan),
                yes_no(input.assets.has_voice_notes)
            ),
        },
        WorkflowStep {
            title: "Understand",
            detail: format!(
                "Segment {} hero scenes across {} and rank quality before scripting the master cut.",
                room_sequence.len(),
                room_sequence.join(", ")
            ),
        },
        WorkflowStep {
            title: "Plan",
            detail: format!(
                "Assemble a long-form walkthrough plus 2 to 3 shorts using Ethan's template system and {} hook framing.",
                hook_style_label(&input.hook_style)
            ),
        },
        WorkflowStep {
            title: "Review",
            detail: format!(
                "Human review is limited to factual verification, title selection, and CTA approval for {}.",
                input.agent_name
            ),
        },
    ];

    let templates = vec![
        build_longform_template(
            &input,
            &room_sequence,
            &price,
            &home_facts,
            &hook,
            shot_span,
        ),
        build_short_template(&input, &room_sequence, &price),
        build_listing_template(&input, &room_sequence, &price),
    ];

    let publish_pack = PublishPack {
        title_candidates: title_candidates(&input, &price),
        description: description(&input, &home_facts),
        chapter_suggestions: chapter_suggestions(&room_sequence),
        thumbnail_options: thumbnail_options(&input, &price),
        export_targets: export_targets(&input),
        subtitle_pack: SubtitlePack {
            primary_language: "English burned-in captions",
            secondary_language: "Simplified Chinese subtitle file",
            styling: "High-contrast editorial captions with restrained lower-thirds",
        },
    };

    let review_notes = review_notes(&input, &room_sequence);

    Ok(ProjectResponse {
        creative_direction,
        workflow,
        templates,
        publish_pack,
        review_notes,
    })
}

fn validate(input: &GenerateRequest) -> Result<(), GenerateError> {
    for (value, field) in [
        (&input.listing_title, "listing_title"),
        (&input.city, "city"),
        (&input.neighborhood, "neighborhood"),
        (&input.agent_name, "agent_name"),
        (&input.buyer_profile, "buyer_profile"),
        (&input.hero_feature, "hero_feature"),
        (&input.cta, "cta"),
        (&input.brand_voice, "brand_voice"),
        (&input.hook_style, "hook_style"),
    ] {
        if value.is_empty() {
            return Err(GenerateError::MissingField(field));
        }
    }

    if input.price_millions <= 0.0 {
        return Err(GenerateError::InvalidPrice);
    }
    if input.assets.footage_clips == 0 {
        return Err(GenerateError::InvalidAssetCount("footage_clips"));
    }
    if input.assets.listing_photos == 0 {
        return Err(GenerateError::InvalidAssetCount("listing_photos"));
    }
    if input.room_sequence.is_empty() {
        return Err(GenerateError::MissingRooms);
    }
    Ok(())
}

fn trim_fields(input: &mut GenerateRequest) {
    for field in [
        &mut input.listing_title,
        &mut input.city,
        &mut input.neighborhood,
        &mut input.agent_name,
        &mut input.buyer_profile,
        &mut input.hero_feature,
        &mut input.cta,
        &mut input.brand_voice,
        &mut input.hook_style,
    ] {
        *field = field.trim().to_string();
    }
    input.room_sequence = input
        .room_sequence
        .iter()
        .map(|room| room.trim().to_string())
        .filter(|room| !room.is_empty())
        .collect();
}

fn normalize_rooms(rooms: &[String]) -> Vec<String> {
    rooms.iter().map(|room| title_case(room)).collect()
}

fn title_case(value: &str) -> String {
    value
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => format!(
                    "{}{}",
                    first.to_uppercase().collect::<String>(),
                    chars.as_str().to_lowercase()
                ),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn price_label(price_millions: f32) -> String {
    format!("${price_millions:.2}M")
}

fn format_baths(value: f32) -> String {
    if (value - value.round()).abs() < f32::EPSILON {
        format!("{}", value.round() as i32)
    } else {
        format!("{value:.1}")
    }
}

fn brand_line(voice: &str, city: &str) -> String {
    match voice {
        "assertive" => format!(
            "A sharper, operator-grade edit language for {} inventory.",
            city
        ),
        "warm" => format!(
            "Editorial warmth with enough polish to feel premium in {}.",
            city
        ),
        _ => format!(
            "Luxury restraint, cinematic pacing, and clean numbers for {}.",
            city
        ),
    }
}

fn palette_for(voice: &str) -> Vec<&'static str> {
    match voice {
        "assertive" => vec!["graphite", "oxide", "bone", "signal amber"],
        "warm" => vec!["sand", "coffee", "cedar", "olive haze"],
        _ => vec!["ink", "champagne", "stone", "soft brass"],
    }
}

fn motion_notes(voice: &str) -> Vec<&'static str> {
    match voice {
        "assertive" => vec![
            "Use hard cuts at the hook and let statistic cards punch in with confidence.",
            "Keep lower-thirds minimal and let room reveals feel decisive.",
        ],
        "warm" => vec![
            "Blend room transitions with gentle crossfades and a lived-in cadence.",
            "Use map and amenity cards as calm pauses rather than aggressive interrupts.",
        ],
        _ => vec![
            "Lead with slow camera confidence, then accelerate once the headline feature lands.",
            "Give every overlay a gallery-like entrance instead of social-style bounce.",
        ],
    }
}

fn opening_hook(input: &GenerateRequest) -> String {
    match input.hook_style.as_str() {
        "numbers" => format!(
            "{} in {} for {} sounds expensive until you see what the first 20 seconds reveal.",
            input.listing_title,
            input.neighborhood,
            price_label(input.price_millions)
        ),
        "lifestyle" => format!(
            "If your buyer wants {} without giving up {} polish, this is the first frame to show.",
            input.buyer_profile, input.city
        ),
        _ => format!(
            "Start on {} and let the home announce itself before the numbers enter.",
            input.hero_feature
        ),
    }
}

fn hook_style_label(hook_style: &str) -> &'static str {
    match hook_style {
        "numbers" => "numbers-first",
        "lifestyle" => "lifestyle-led",
        _ => "cinematic",
    }
}

fn longform_shot_span(room_count: usize) -> String {
    let seconds = 230 + room_count as u32 * 24;
    format!("{}:{:02}", seconds / 60, seconds % 60)
}

fn build_longform_template(
    input: &GenerateRequest,
    rooms: &[String],
    price: &str,
    home_facts: &str,
    hook: &str,
    duration_label: String,
) -> TemplateOutput {
    let mut shot_plan = vec![
        ShotPlan {
            label: "Cold Open".to_string(),
            duration_label: "0:00-0:12".to_string(),
            purpose: "Land the curiosity hook before any overt selling begins.".to_string(),
            source: if input.assets.has_drone {
                "drone + exterior arrival".to_string()
            } else {
                "best exterior approach shot".to_string()
            },
            caption: hook.to_string(),
        },
        ShotPlan {
            label: "Orientation".to_string(),
            duration_label: "0:12-0:35".to_string(),
            purpose: "Place the home in neighborhood context and establish the buyer promise."
                .to_string(),
            source: if input.assets.has_neighborhood_broll {
                "neighborhood b-roll + front entry".to_string()
            } else {
                "front entry + first interior reveal".to_string()
            },
            caption: format!("{} · {}", input.neighborhood, home_facts),
        },
    ];

    for (index, room) in rooms.iter().enumerate() {
        let start = 35 + index as u32 * 26;
        let end = start + 24;
        shot_plan.push(ShotPlan {
            label: format!("{} Sequence", room),
            duration_label: format!(
                "{}:{:02}-{}:{:02}",
                start / 60,
                start % 60,
                end / 60,
                end % 60
            ),
            purpose: format!(
                "Show why the {} is part of the selling argument, not filler coverage.",
                room
            ),
            source: format!("best {} clips", room.to_lowercase()),
            caption: format!("Anchor the edit on {}.", room.to_lowercase()),
        });
    }

    let tradeoff_start = 35 + rooms.len() as u32 * 26;
    let tradeoff_end = tradeoff_start + 28;
    shot_plan.push(ShotPlan {
        label: "Numbers and Tradeoffs".to_string(),
        duration_label: format!(
            "{}:{:02}-{}:{:02}",
            tradeoff_start / 60,
            tradeoff_start % 60,
            tradeoff_end / 60,
            tradeoff_end % 60
        ),
        purpose: "Earn trust with price framing, positioning, and one honest concession."
            .to_string(),
        source: "detail shots + data card".to_string(),
        caption: format!("{} with room for {}.", price, input.buyer_profile),
    });
    shot_plan.push(ShotPlan {
        label: "Close".to_string(),
        duration_label: format!(
            "{}:{:02}-{}:{:02}",
            tradeoff_end / 60,
            tradeoff_end % 60,
            (tradeoff_end + 18) / 60,
            (tradeoff_end + 18) % 60
        ),
        purpose: "Finish with a composed CTA instead of an over-explained outro.".to_string(),
        source: "best return shot + agent sign-off".to_string(),
        caption: input.cta.clone(),
    });

    TemplateOutput {
        slug: "open-house-masterpiece",
        name: "Open House Masterpiece",
        duration_label,
        aspect_ratio: "16:9",
        opening_hook: hook.to_string(),
        summary: format!(
            "A composed long-form walkthrough for {} that sells the home through rhythm, selective data, and confident restraint.",
            input.buyer_profile
        ),
        shot_plan,
        overlays: vec![
            format!("Price card: {}", price),
            format!("Home facts strip: {}", home_facts),
            format!("Agent signature line: {}", input.agent_name),
            "Closing CTA frame with inquiry prompt".to_string(),
        ],
    }
}

fn build_short_template(input: &GenerateRequest, rooms: &[String], price: &str) -> TemplateOutput {
    let featured_room = rooms
        .first()
        .cloned()
        .unwrap_or_else(|| "Arrival".to_string());
    TemplateOutput {
        slug: "hero-short",
        name: "Hero Short",
        duration_label: "0:31".to_string(),
        aspect_ratio: "9:16",
        opening_hook: format!(
            "Stop on the {} and let viewers earn the rest of the house.",
            input.hero_feature
        ),
        summary: format!(
            "A vertical cut optimized for scroll-stopping impact around {} and a crisp price reveal.",
            featured_room.to_lowercase()
        ),
        shot_plan: vec![
            ShotPlan {
                label: "Scroll Stop".to_string(),
                duration_label: "0:00-0:05".to_string(),
                purpose: "Hit the single strongest visual before text overload.".to_string(),
                source: format!("best {} vertical crop", featured_room.to_lowercase()),
                caption: input.hero_feature.clone(),
            },
            ShotPlan {
                label: "Value Compression".to_string(),
                duration_label: "0:05-0:18".to_string(),
                purpose: "Stack the most memorable facts with almost no dead air.".to_string(),
                source: "two to three premium room cuts".to_string(),
                caption: format!("{} · {} · {}", input.neighborhood, price, input.beds),
            },
            ShotPlan {
                label: "Direct CTA".to_string(),
                duration_label: "0:18-0:31".to_string(),
                purpose: "Convert curiosity into a showing or DM.".to_string(),
                source: "agent sign-off or closing detail".to_string(),
                caption: input.cta.clone(),
            },
        ],
        overlays: vec![
            "Vertical title lockup".to_string(),
            format!("Price badge: {}", price),
            "One-line CTA footer".to_string(),
        ],
    }
}

fn build_listing_template(
    input: &GenerateRequest,
    rooms: &[String],
    price: &str,
) -> TemplateOutput {
    let room_rollup = rooms.iter().take(3).cloned().collect::<Vec<_>>().join(", ");
    TemplateOutput {
        slug: "listing-explainer",
        name: "Listing Explainer",
        duration_label: "1:12".to_string(),
        aspect_ratio: "4:5",
        opening_hook: format!(
            "{} in {}: the concise version for warm leads.",
            input.listing_title, input.neighborhood
        ),
        summary: "A denser asset for reshares, private messages, and high-intent prospects."
            .to_string(),
        shot_plan: vec![
            ShotPlan {
                label: "Premise".to_string(),
                duration_label: "0:00-0:12".to_string(),
                purpose: "State the home's lane immediately.".to_string(),
                source: "cleanest hero shot".to_string(),
                caption: format!("{} · {}", input.listing_title, price),
            },
            ShotPlan {
                label: "Proof".to_string(),
                duration_label: "0:12-0:44".to_string(),
                purpose: "Show the rooms that justify the ask.".to_string(),
                source: format!("coverage from {}", room_rollup),
                caption: format!("{} for {}", input.hero_feature, input.buyer_profile),
            },
            ShotPlan {
                label: "Action".to_string(),
                duration_label: "0:44-1:12".to_string(),
                purpose: "Package the next step like a concierge invite, not a generic lead form."
                    .to_string(),
                source: "best detail + contact frame".to_string(),
                caption: input.cta.clone(),
            },
        ],
        overlays: vec![
            "Factual lower-third stack".to_string(),
            "Commute / neighborhood tag".to_string(),
            "Private share CTA".to_string(),
        ],
    }
}

fn title_candidates(input: &GenerateRequest, price: &str) -> Vec<String> {
    vec![
        format!(
            "{}: {} in {}",
            input.listing_title, input.hero_feature, input.neighborhood
        ),
        format!(
            "Inside a {} home for {} in {}",
            input.buyer_profile, price, input.city
        ),
        format!(
            "Why {} buyers keep chasing homes like this in {}",
            input.buyer_profile, input.neighborhood
        ),
    ]
}

fn description(input: &GenerateRequest, home_facts: &str) -> String {
    format!(
        "Ethan AI edit pack for {} in {}. This cut is designed for {} buyers and leads with {}. Facts: {}. CTA: {}. Operator note: keep the script grounded, show one honest tradeoff, and preserve a premium tone throughout.",
        input.listing_title,
        input.neighborhood,
        input.buyer_profile,
        input.hero_feature,
        home_facts,
        input.cta
    )
}

fn chapter_suggestions(rooms: &[String]) -> Vec<String> {
    let mut chapters = vec![
        "00:00 The hook".to_string(),
        "00:12 Neighborhood read".to_string(),
    ];
    for (index, room) in rooms.iter().enumerate() {
        chapters.push(format!(
            "0{}:{:02} {}",
            1 + index / 2,
            5 + (index * 18) % 50,
            room
        ));
    }
    chapters.push("03:58 Numbers and tradeoffs".to_string());
    chapters.push("04:24 Final CTA".to_string());
    chapters
}

fn thumbnail_options(input: &GenerateRequest, price: &str) -> Vec<String> {
    vec![
        format!("{} / {}", input.hero_feature, price),
        format!("{} buyers, look here", input.buyer_profile),
        format!("{} in {}", input.listing_title, input.neighborhood),
    ]
}

fn export_targets(input: &GenerateRequest) -> Vec<ExportTarget> {
    vec![
        ExportTarget {
            label: "YouTube long-form",
            delivery: format!(
                "16:9 master with chapter suggestions and {}",
                input.agent_name
            ),
        },
        ExportTarget {
            label: "Shorts pack",
            delivery: "2 to 3 vertical cuts with burned-in captions".to_string(),
        },
        ExportTarget {
            label: "Private-share explainer",
            delivery: "4:5 cut for DMs, email, and reposts".to_string(),
        },
    ]
}

fn review_notes(input: &GenerateRequest, rooms: &[String]) -> Vec<String> {
    let mut notes = vec![
        format!(
            "Keep {} as the first emotional reveal. That is the core click-to-tour bridge.",
            input.hero_feature
        ),
        format!(
            "Use {} as the factual anchor so the edit feels confident instead of salesy.",
            price_label(input.price_millions)
        ),
    ];

    if !input.assets.has_drone {
        notes.push("No drone footage detected. Use a slower front-entry sequence instead of faking a cinematic aerial reveal.".to_string());
    }
    if !input.assets.has_voice_notes {
        notes.push("No voice notes provided. Script the narration from listing facts only and avoid assumptions about renovation history.".to_string());
    }
    if input.assets.footage_clips < 8 {
        notes.push("Clip count is thin for a premium cut. Favor fewer, cleaner rooms rather than padding the timeline.".to_string());
    }
    if rooms.len() < 3 {
        notes.push("Room coverage is narrow. Lean into a boutique-story angle instead of pretending this is a full home tour.".to_string());
    }

    notes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_request() -> GenerateRequest {
        GenerateRequest {
            listing_title: "Belmont House".to_string(),
            city: "San Mateo".to_string(),
            neighborhood: "Baywood".to_string(),
            price_millions: 3.6,
            beds: 4,
            baths: 3.5,
            sqft: 3180,
            agent_name: "Olivia Chen".to_string(),
            buyer_profile: "design-conscious family".to_string(),
            hero_feature: "double-height living room".to_string(),
            cta: "Book a private showing with Olivia Chen".to_string(),
            brand_voice: "luxury".to_string(),
            hook_style: "cinematic".to_string(),
            room_sequence: vec![
                "arrival".to_string(),
                "living room".to_string(),
                "kitchen".to_string(),
            ],
            assets: AssetManifest {
                footage_clips: 18,
                listing_photos: 24,
                has_floorplan: true,
                has_voice_notes: true,
                has_drone: true,
                has_neighborhood_broll: true,
            },
        }
    }

    #[test]
    fn generate_returns_three_templates() {
        let response = generate(sample_request()).expect("request should be valid");
        assert_eq!(response.templates.len(), 3);
        assert_eq!(response.publish_pack.title_candidates.len(), 3);
    }

    #[test]
    fn generate_requires_rooms() {
        let mut request = sample_request();
        request.room_sequence.clear();
        let error = generate(request).expect_err("request should fail");
        assert!(matches!(error, GenerateError::MissingRooms));
    }
}
