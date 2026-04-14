(function () {
  requestAnimationFrame(() => {
    document.body.classList.add("is-ready");
  });

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
  const promptPreview = document.getElementById("promptPreview");
  const inspectorFacts = document.getElementById("inspectorFacts");
  const outputChecklist = document.getElementById("outputChecklist");
  const runSnapshot = document.getElementById("runSnapshot");
  const approvalChecklist = document.getElementById("approvalChecklist");

  const summary = {
    listingTitle: document.getElementById("summaryListingTitle"),
    listingMeta: document.getElementById("summaryListingMeta"),
    coverageTitle: document.getElementById("summaryCoverageTitle"),
    coverageMeta: document.getElementById("summaryCoverageMeta"),
    deliveryTitle: document.getElementById("summaryDeliveryTitle"),
    deliveryMeta: document.getElementById("summaryDeliveryMeta"),
  };

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

  init();

  async function init() {
    await loadTemplateCatalog();
    sampleButton.addEventListener("click", loadSample);
    form.addEventListener("submit", handleSubmit);
    form.addEventListener("input", syncDraftPanels);
    form.addEventListener("change", syncDraftPanels);
    loadSample();
  }

  async function loadTemplateCatalog() {
    try {
      const response = await fetch("/api/templates");
      const templates = await response.json();
      templateStrip.innerHTML = "";
      templates.forEach((template) => {
        const card = createElement("article", "template-shell");
        const label = createElement(
          "span",
          "template-shell-tag",
          `${template.aspect_ratio} / ${template.duration_label}`
        );
        const title = createElement("h3", "", template.name);
        const promise = createElement("p", "", template.promise);
        card.append(label, title, promise);
        templateStrip.appendChild(card);
      });
    } catch (error) {
      templateStrip.innerHTML = "";
      const fallback = createElement("article", "template-shell");
      fallback.append(
        createElement("span", "template-shell-tag", "Catalog unavailable"),
        createElement("h3", "", "Template catalog unavailable"),
        createElement("p", "", "The app can still generate a campaign packet once the API responds.")
      );
      templateStrip.appendChild(fallback);
    }
  }

  function loadSample() {
    Object.entries(sample).forEach(([key, value]) => {
      const element = form.elements.namedItem(key);
      if (!element || element instanceof RadioNodeList) {
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

    syncDraftPanels();
    message.textContent = "Sample listing loaded. Review the brief or run the packet.";
  }

  async function handleSubmit(event) {
    event.preventDefault();
    submitButton.disabled = true;
    submitButton.textContent = "Generating...";
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

      renderResults(payload, data);
      message.textContent = "Campaign packet generated.";
    } catch (error) {
      message.textContent = "The Ethan run failed.";
    } finally {
      submitButton.disabled = false;
      submitButton.textContent = "Generate Campaign Packet";
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

  function syncDraftPanels() {
    const draft = collectPayload();
    const roomCount = draft.room_sequence.length || 0;
    const listingLabel = draft.listing_title || "Untitled listing";
    const locationLabel = [draft.city, draft.neighborhood].filter(Boolean).join(" / ") || "Location pending";

    promptPreview.textContent = [
      `Create a ${hookStyleLabel(draft.hook_style)} flagship walkthrough for ${listingLabel}.`,
      draft.hero_feature ? `Lead with ${draft.hero_feature}.` : "Pick a strong opening feature.",
      draft.buyer_profile ? `Keep the buyer fit pointed at ${draft.buyer_profile}.` : "Set the buyer profile.",
      draft.cta ? `End with: ${draft.cta}.` : "Add a clear CTA.",
    ].join(" ");

    summary.listingTitle.textContent = listingLabel;
    summary.listingMeta.textContent = `${locationLabel} / ${draft.beds || "-"} bed / ${formatBaths(draft.baths)} bath / ${draft.sqft || "-"} sq ft / ${formatPrice(draft.price_millions)}`;
    summary.coverageTitle.textContent = `${roomCount || 0} anchor scenes`;
    summary.coverageMeta.textContent = `${draft.assets.footage_clips || 0} clips / ${draft.assets.listing_photos || 0} photos / ${flagSummary(draft)}`;
    summary.deliveryTitle.textContent = "3 outputs + publish pack";
    summary.deliveryMeta.textContent = `${brandVoiceLabel(draft.brand_voice)} / ${hookStyleLabel(draft.hook_style)} / agent ${draft.agent_name || "pending"}`;

    renderSimpleList(inspectorFacts, [
      `${listingLabel} in ${locationLabel}`,
      `${draft.beds || 0} bed / ${formatBaths(draft.baths)} bath / ${draft.sqft || 0} sq ft / ${formatPrice(draft.price_millions)}`,
      roomCount > 0 ? `${roomCount} narrative scenes selected: ${draft.room_sequence.map(titleCase).join(", ")}` : "No narrative scenes selected yet",
      draft.hero_feature ? `Hero feature: ${draft.hero_feature}` : "Hero feature not set",
      draft.buyer_profile ? `Buyer fit: ${draft.buyer_profile}` : "Buyer profile not set",
    ]);

    renderSimpleList(outputChecklist, [
      "Open House Masterpiece walkthrough",
      "Hero Short vertical cut",
      "Listing Explainer social asset",
      "Title, thumbnail, chapter, subtitle, and export pack",
    ]);

    renderSimpleList(approvalChecklist, [
      "Confirm all listing facts and pricing language",
      "Choose the final title and thumbnail",
      "Approve the CTA wording and tone",
      "Catch any market nuance Ethan should not guess",
    ]);
  }

  function renderResults(payload, data) {
    emptyState.classList.add("hidden");
    resultsStack.classList.remove("hidden");

    output.creativeHeadline.textContent = data.creative_direction.headline;
    output.brandLine.textContent = data.creative_direction.brand_line;
    output.operatorBrief.textContent = data.creative_direction.operator_brief;

    renderTokenList(output.paletteTokens, data.creative_direction.palette);
    renderSimpleList(output.motionNotes, data.creative_direction.motion_notes);

    renderRunSnapshot(payload, data);
    renderWorkflow(data.workflow);
    renderTemplateOutputs(data.templates);
    renderSimpleList(output.titleCandidates, data.publish_pack.title_candidates);
    renderSimpleList(output.thumbnailOptions, data.publish_pack.thumbnail_options);
    renderSimpleList(output.chapterSuggestions, data.publish_pack.chapter_suggestions);
    renderExportTargets(data.publish_pack.export_targets);

    output.descriptionText.textContent = data.publish_pack.description;
    output.subtitlePack.textContent = `${data.publish_pack.subtitle_pack.primary_language} / ${data.publish_pack.subtitle_pack.secondary_language} / ${data.publish_pack.subtitle_pack.styling}`;
    renderSimpleList(output.reviewNotes, data.review_notes);
  }

  function renderRunSnapshot(payload, data) {
    const missingAssets = [];
    if (!payload.assets.has_floorplan) {
      missingAssets.push("floorplan");
    }
    if (!payload.assets.has_voice_notes) {
      missingAssets.push("voice notes");
    }
    if (!payload.assets.has_drone) {
      missingAssets.push("drone");
    }
    if (!payload.assets.has_neighborhood_broll) {
      missingAssets.push("neighborhood b-roll");
    }

    renderSimpleList(runSnapshot, [
      `${data.templates.length} template outputs generated from ${payload.room_sequence.length} selected scenes`,
      `${payload.assets.footage_clips} clips and ${payload.assets.listing_photos} listing photos in the run`,
      missingAssets.length > 0
        ? `Missing optional assets: ${missingAssets.join(", ")}`
        : "All optional support assets are present",
      `Primary reviewer: ${payload.agent_name} will confirm facts, title, and CTA`,
    ]);
  }

  function renderWorkflow(items) {
    output.workflowList.innerHTML = "";
    items.forEach((step, index) => {
      const item = createElement("article", "workflow-item");
      const head = createElement("div", "workflow-item-head");
      head.append(
        createElement("span", "workflow-number", String(index + 1).padStart(2, "0")),
        createElement("h3", "", step.title)
      );
      item.append(head, createElement("p", "", step.detail));
      output.workflowList.appendChild(item);
    });
  }

  function renderTemplateOutputs(templates) {
    output.templateOutputs.innerHTML = "";
    templates.forEach((template) => {
      const article = createElement("article", "template-output");

      const head = createElement("div", "template-output-head");
      const titleWrap = createElement("div");
      titleWrap.append(
        createElement("h3", "", template.name),
        createElement("p", "", template.summary)
      );
      head.append(
        titleWrap,
        createElement("span", "template-shell-tag", `${template.aspect_ratio} / ${template.duration_label}`)
      );

      const hook = createElement("p", "");
      hook.append(
        createStrongText("Opening hook: "),
        document.createTextNode(template.opening_hook)
      );

      const shotPlan = createElement("div", "shot-plan");
      template.shot_plan.forEach((shot) => {
        const row = createElement("article", "shot-row");
        const meta = createElement("div");
        meta.append(
          createElement("strong", "", shot.label),
          createElement("p", "", shot.purpose),
          createElement("p", "shot-caption", `${shot.source} / ${shot.caption}`)
        );
        row.append(createElement("time", "", shot.duration_label), meta);
        shotPlan.appendChild(row);
      });

      const overlayTitle = createElement("span", "workspace-label", "Overlays");
      const overlayList = createElement("ul", "overlay-list");
      template.overlays.forEach((entry) => {
        overlayList.appendChild(createElement("li", "", entry));
      });

      article.append(head, hook, shotPlan, overlayTitle, overlayList);
      output.templateOutputs.appendChild(article);
    });
  }

  function renderExportTargets(items) {
    output.exportTargets.innerHTML = "";
    items.forEach((entry) => {
      const item = createElement("li", "");
      item.append(createStrongText(`${entry.label}: `), document.createTextNode(entry.delivery));
      output.exportTargets.appendChild(item);
    });
  }

  function renderSimpleList(container, items) {
    container.innerHTML = "";
    items.forEach((text) => {
      container.appendChild(createElement("li", "", text));
    });
  }

  function renderTokenList(container, items) {
    container.innerHTML = "";
    items.forEach((text) => {
      container.appendChild(createElement("span", "token", text));
    });
  }

  function createElement(tag, className, text) {
    const element = document.createElement(tag);
    if (className) {
      element.className = className;
    }
    if (typeof text === "string") {
      element.textContent = text;
    }
    return element;
  }

  function createStrongText(text) {
    return createElement("strong", "", text);
  }

  function hookStyleLabel(value) {
    switch (value) {
      case "numbers":
        return "numbers-first";
      case "lifestyle":
        return "lifestyle-led";
      default:
        return "cinematic";
    }
  }

  function brandVoiceLabel(value) {
    switch (value) {
      case "warm":
        return "Warm editorial";
      case "assertive":
        return "Operator sharp";
      default:
        return "Luxury restraint";
    }
  }

  function formatBaths(value) {
    const number = Number(value || 0);
    if (Number.isInteger(number)) {
      return String(number);
    }
    return number.toFixed(1);
  }

  function formatPrice(value) {
    const number = Number(value || 0);
    if (number <= 0) {
      return "price pending";
    }
    return `$${number.toFixed(1)}M`;
  }

  function titleCase(text) {
    return String(text || "")
      .split(" ")
      .filter(Boolean)
      .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
      .join(" ");
  }

  function flagSummary(payload) {
    const flags = [];
    if (payload.assets.has_floorplan) {
      flags.push("floorplan");
    }
    if (payload.assets.has_drone) {
      flags.push("drone");
    }
    if (payload.assets.has_neighborhood_broll) {
      flags.push("b-roll");
    }
    return flags.length > 0 ? flags.join(" / ") : "core assets only";
  }
})();
