(function () {
  const page = document.body.dataset.page;

  if (page !== "studio") {
    return;
  }

  const form = document.getElementById("plannerForm");
  const sampleButton = document.getElementById("sampleButton");
  const submitButton = document.getElementById("submitButton");
  const message = document.getElementById("formMessage");
  const emptyState = document.getElementById("emptyState");
  const resultsStack = document.getElementById("resultsStack");
  const templateStrip = document.getElementById("templateStrip");

  const output = {
    creativeHeadline: document.getElementById("creativeHeadline"),
    brandLine: document.getElementById("brandLine"),
    operatorBrief: document.getElementById("operatorBrief"),
    paletteTokens: document.getElementById("paletteTokens"),
    motionNotes: document.getElementById("motionNotes"),
    workflowList: document.getElementById("workflowList"),
    templateOutputs: document.getElementById("templateOutputs"),
    titleCandidates: document.getElementById("titleCandidates"),
    thumbnailOptions: document.getElementById("thumbnailOptions"),
    chapterSuggestions: document.getElementById("chapterSuggestions"),
    exportTargets: document.getElementById("exportTargets"),
    descriptionText: document.getElementById("descriptionText"),
    subtitlePack: document.getElementById("subtitlePack"),
    reviewNotes: document.getElementById("reviewNotes"),
  };

  const sample = {
    listing_title: "Belmont House",
    city: "San Mateo",
    neighborhood: "Baywood",
    price_millions: 3.6,
    beds: 4,
    baths: 3.5,
    sqft: 3180,
    agent_name: "Olivia Chen",
    buyer_profile: "design-conscious family relocating from SF",
    hero_feature: "double-height living room",
    cta: "Book a private showing with Olivia Chen",
    brand_voice: "luxury",
    hook_style: "cinematic",
    room_sequence: ["arrival", "living room", "kitchen", "primary suite"],
    footage_clips: 18,
    listing_photos: 24,
    has_floorplan: true,
    has_voice_notes: true,
    has_drone: true,
    has_neighborhood_broll: true,
  };

  async function init() {
    await loadTemplateCatalog();
    sampleButton.addEventListener("click", loadSample);
    form.addEventListener("submit", handleSubmit);
    loadSample();
  }

  async function loadTemplateCatalog() {
    try {
      const response = await fetch("/api/templates");
      const templates = await response.json();
      templateStrip.innerHTML = "";
      templates.forEach((template) => {
        const pill = document.createElement("div");
        pill.className = "template-pill";
        pill.textContent = `${template.name} · ${template.aspect_ratio} · ${template.duration_label}`;
        templateStrip.appendChild(pill);
      });
    } catch (error) {
      templateStrip.innerHTML = '<div class="template-pill">Template catalog unavailable</div>';
    }
  }

  function loadSample() {
    Object.entries(sample).forEach(([key, value]) => {
      const element = form.elements.namedItem(key);
      if (!element) {
        return;
      }

      if (element instanceof RadioNodeList) {
        return;
      }

      if (element.type === "checkbox") {
        element.checked = Boolean(value);
        return;
      }

      element.value = value;
    });

    const roomInputs = form.querySelectorAll('input[name="room_sequence"]');
    roomInputs.forEach((input) => {
      input.checked = sample.room_sequence.includes(input.value);
    });

    message.textContent = "Sample listing loaded. Generate the Ethan packet.";
  }

  async function handleSubmit(event) {
    event.preventDefault();
    submitButton.disabled = true;
    submitButton.textContent = "Generating…";
    message.textContent = "Ethan is composing the campaign packet.";

    const payload = collectPayload();

    try {
      const response = await fetch("/api/plan", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });
      const data = await response.json();

      if (!response.ok) {
        message.textContent = data.error || "The Ethan run failed.";
        return;
      }

      renderResults(data);
      message.textContent = "Campaign packet generated.";
    } catch (error) {
      message.textContent = "The Ethan run failed.";
    } finally {
      submitButton.disabled = false;
      submitButton.textContent = "Generate Ethan Plan";
    }
  }

  function collectPayload() {
    const formData = new FormData(form);
    return {
      listing_title: String(formData.get("listing_title") || "").trim(),
      city: String(formData.get("city") || "").trim(),
      neighborhood: String(formData.get("neighborhood") || "").trim(),
      price_millions: Number(formData.get("price_millions") || 0),
      beds: Number(formData.get("beds") || 0),
      baths: Number(formData.get("baths") || 0),
      sqft: Number(formData.get("sqft") || 0),
      agent_name: String(formData.get("agent_name") || "").trim(),
      buyer_profile: String(formData.get("buyer_profile") || "").trim(),
      hero_feature: String(formData.get("hero_feature") || "").trim(),
      cta: String(formData.get("cta") || "").trim(),
      brand_voice: String(formData.get("brand_voice") || "").trim(),
      hook_style: String(formData.get("hook_style") || "").trim(),
      room_sequence: formData.getAll("room_sequence").map((value) => String(value)),
      assets: {
        footage_clips: Number(formData.get("footage_clips") || 0),
        listing_photos: Number(formData.get("listing_photos") || 0),
        has_floorplan: Boolean(formData.get("has_floorplan")),
        has_voice_notes: Boolean(formData.get("has_voice_notes")),
        has_drone: Boolean(formData.get("has_drone")),
        has_neighborhood_broll: Boolean(formData.get("has_neighborhood_broll")),
      },
    };
  }

  function renderResults(data) {
    emptyState.classList.add("hidden");
    resultsStack.classList.remove("hidden");

    output.creativeHeadline.textContent = data.creative_direction.headline;
    output.brandLine.textContent = data.creative_direction.brand_line;
    output.operatorBrief.textContent = data.creative_direction.operator_brief;
    renderList(output.paletteTokens, data.creative_direction.palette, createToken);
    renderList(output.motionNotes, data.creative_direction.motion_notes, createListItem);

    renderList(output.workflowList, data.workflow, (step) => {
      const item = document.createElement("div");
      item.className = "workflow-item";
      item.innerHTML = `<span>${step.title}</span><p>${step.detail}</p>`;
      return item;
    });

    renderList(output.templateOutputs, data.templates, (template) => {
      const wrapper = document.createElement("article");
      wrapper.className = "template-output";
      const shots = template.shot_plan
        .map(
          (shot) => `
            <div class="shot-row">
              <time>${shot.duration_label}</time>
              <div>
                <strong>${shot.label}</strong>
                <p>${shot.purpose}</p>
                <small>${shot.source} · ${shot.caption}</small>
              </div>
            </div>
          `
        )
        .join("");
      const overlays = template.overlays.map((overlay) => `<li>${overlay}</li>`).join("");
      wrapper.innerHTML = `
        <div class="template-output-head">
          <div>
            <h4>${template.name}</h4>
            <p>${template.summary}</p>
          </div>
          <span class="mono-tag">${template.aspect_ratio} · ${template.duration_label}</span>
        </div>
        <p><strong>Opening hook:</strong> ${template.opening_hook}</p>
        <div>${shots}</div>
        <ul class="clean-list">${overlays}</ul>
      `;
      return wrapper;
    });

    renderList(output.titleCandidates, data.publish_pack.title_candidates, createListItem);
    renderList(output.thumbnailOptions, data.publish_pack.thumbnail_options, createListItem);
    renderList(output.chapterSuggestions, data.publish_pack.chapter_suggestions, createListItem);
    renderList(output.exportTargets, data.publish_pack.export_targets, (entry) => {
      const item = document.createElement("li");
      item.textContent = `${entry.label}: ${entry.delivery}`;
      return item;
    });
    output.descriptionText.textContent = data.publish_pack.description;
    output.subtitlePack.textContent = `${data.publish_pack.subtitle_pack.primary_language} / ${data.publish_pack.subtitle_pack.secondary_language} / ${data.publish_pack.subtitle_pack.styling}`;
    renderList(output.reviewNotes, data.review_notes, createListItem);
  }

  function renderList(container, items, renderItem) {
    container.innerHTML = "";
    items.forEach((item) => container.appendChild(renderItem(item)));
  }

  function createListItem(text) {
    const item = document.createElement("li");
    item.textContent = text;
    return item;
  }

  function createToken(text) {
    const token = document.createElement("span");
    token.className = "token";
    token.textContent = text;
    return token;
  }

  init();
})();
