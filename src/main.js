const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const slotsContainer = document.getElementById("slots");
const addressSection = document.getElementById("address-section");
const enableToggle = document.getElementById("enableToggle");
const toggleLabel = document.querySelector(".toggle-label");
const saveBtn = document.getElementById("saveBtn");
const newAddrBtn = document.getElementById("newAddrBtn");

// Track which keybind button is currently listening for input
let listeningBtn = null;
let listeningCallback = null;

// ---- Slots ----

async function loadSlots() {
  const slots = await invoke("get_slots");
  renderSlots(slots);
}

function renderSlots(slots) {
  slotsContainer.innerHTML = "";
  slots.forEach((slot) => {
    const div = document.createElement("div");
    div.className = "slot";

    const keybinds = document.createElement("div");
    keybinds.className = "slot-keybinds";

    const pasteBtn = createKeybindBtn(
      slot.paste_shortcut,
      "Paste keybind (Esc to clear)",
      (shortcut) => invoke("set_shortcut", { id: slot.id, kind: "paste", shortcut })
    );

    const copyBtn = createKeybindBtn(
      slot.copy_shortcut,
      "Copy-to-slot keybind (Esc to clear)",
      (shortcut) => invoke("set_shortcut", { id: slot.id, kind: "copy", shortcut }),
      true
    );

    keybinds.appendChild(pasteBtn);
    keybinds.appendChild(copyBtn);

    const input = document.createElement("input");
    input.className = "slot-input";
    input.type = "text";
    input.value = slot.content;
    input.placeholder = "Slot " + slot.id;
    input.dataset.id = slot.id;
    input.addEventListener("input", () => {
      invoke("set_slot", { id: slot.id, content: input.value });
    });

    div.appendChild(keybinds);
    div.appendChild(input);
    slotsContainer.appendChild(div);
  });
}

// ---- Address ----

async function loadAddress() {
  const [addr, config] = await Promise.all([
    invoke("get_address"),
    invoke("get_address_config"),
  ]);
  renderAddress(addr, config);
}

function renderAddress(addr, config) {
  addressSection.innerHTML = "";

  const fields = [
    { key: "street", label: "Street", value: addr.street, shortcut: config.street_shortcut },
    { key: "city", label: "City", value: addr.city, shortcut: config.city_shortcut },
    { key: "state", label: "State", value: addr.state, shortcut: config.state_shortcut },
    { key: "zip", label: "Zip", value: addr.zip, shortcut: config.zip_shortcut },
  ];

  fields.forEach((f) => {
    const row = document.createElement("div");
    row.className = "addr-row";

    const btn = createKeybindBtn(
      f.shortcut,
      `Paste ${f.label} keybind (Esc to clear)`,
      (shortcut) => invoke("set_address_shortcut", { field: f.key, shortcut })
    );

    const label = document.createElement("span");
    label.className = "addr-label";
    label.textContent = f.label + ":";

    const value = document.createElement("span");
    value.className = "addr-value";
    value.textContent = f.value;

    row.appendChild(btn);
    row.appendChild(label);
    row.appendChild(value);
    addressSection.appendChild(row);
  });
}

newAddrBtn.addEventListener("click", async () => {
  const addr = await invoke("new_random_address");
  const config = await invoke("get_address_config");
  renderAddress(addr, config);
});

// ---- Shared keybind button ----

function createKeybindBtn(shortcutStr, title, onSet, isCopy) {
  const btn = document.createElement("button");
  btn.className = "keybind-btn" + (isCopy ? " keybind-copy" : "");
  btn.textContent = shortcutStr || "None";
  btn.title = title;
  btn.addEventListener("click", () => {
    if (listeningBtn === btn) {
      cancelListening();
      return;
    }
    cancelListening();
    listeningBtn = btn;
    listeningCallback = onSet;
    btn.classList.add("listening");
    btn.textContent = "Press keys...";
  });
  return btn;
}

function cancelListening() {
  if (listeningBtn) {
    listeningBtn.classList.remove("listening");
  }
  listeningBtn = null;
  listeningCallback = null;
}

// ---- Key capture ----

function codeToKey(code, key) {
  if (code.startsWith("Digit")) return code.replace("Digit", "");
  if (code.startsWith("Numpad") && code.length === 7) return code.replace("Numpad", "Num");
  if (code.startsWith("Key")) return code.replace("Key", "").toUpperCase();
  if (/^F\d+$/.test(code)) return code;
  const map = {
    Space: "Space", Tab: "Tab", Enter: "Enter", Backspace: "Backspace",
    Delete: "Delete", Insert: "Insert", Home: "Home", End: "End",
    PageUp: "PageUp", PageDown: "PageDown",
    ArrowUp: "Up", ArrowDown: "Down", ArrowLeft: "Left", ArrowRight: "Right",
    BracketLeft: "[", BracketRight: "]", Backslash: "\\",
    Semicolon: ";", Quote: "'", Comma: ",", Period: ".", Slash: "/",
    Minus: "-", Equal: "=", Backquote: "`",
  };
  return map[code] || key;
}

document.addEventListener("keydown", async (e) => {
  if (!listeningBtn) return;
  e.preventDefault();
  e.stopPropagation();

  // Escape clears the keybind
  if (e.code === "Escape") {
    const cb = listeningCallback;
    cancelListening();
    try { await cb(""); } catch (err) { console.error(err); }
    loadSlots();
    loadAddress();
    return;
  }

  // Ignore modifier-only presses
  const modOnlyCodes = [
    "ShiftLeft","ShiftRight","ControlLeft","ControlRight",
    "AltLeft","AltRight","MetaLeft","MetaRight"
  ];
  if (modOnlyCodes.includes(e.code)) return;

  const parts = [];
  if (e.ctrlKey) parts.push("Ctrl");
  if (e.altKey) parts.push("Alt");
  if (e.shiftKey) parts.push("Shift");
  if (e.metaKey) parts.push("Super");

  const key = codeToKey(e.code, e.key);
  if (!key) return;
  parts.push(key);

  const shortcut = parts.join("+");
  const cb = listeningCallback;
  cancelListening();

  try {
    await cb(shortcut);
  } catch (err) {
    console.error("Failed to set shortcut:", err);
  }
  loadSlots();
  loadAddress();
});

document.addEventListener("mousedown", (e) => {
  if (listeningBtn && !e.target.classList.contains("keybind-btn")) {
    cancelListening();
    loadSlots();
    loadAddress();
  }
});

// ---- Controls ----

saveBtn.addEventListener("click", async () => {
  await invoke("save_slots");
  saveBtn.textContent = "Saved!";
  setTimeout(() => { saveBtn.textContent = "Save"; }, 1500);
});

enableToggle.addEventListener("change", async () => {
  const enabled = enableToggle.checked;
  await invoke("set_enabled", { enabled });
  toggleLabel.textContent = enabled ? "Enabled" : "Disabled";
});

listen("slots-updated", (event) => { renderSlots(event.payload); });
listen("enabled-changed", (event) => {
  enableToggle.checked = event.payload;
  toggleLabel.textContent = event.payload ? "Enabled" : "Disabled";
});

// ---- Init ----
(async () => {
  await loadSlots();
  await loadAddress();
  const enabled = await invoke("get_enabled");
  enableToggle.checked = enabled;
  toggleLabel.textContent = enabled ? "Enabled" : "Disabled";
})();
