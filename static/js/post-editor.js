const inputTitle = document.querySelector('input[name="title"]');
const inputSlug = document.querySelector('input[name="slug"]');
const inputDt = document.querySelector('input[name="creation_datetime"]');
const inputSubtitle = document.querySelector('input[name="subtitle"]');
const inputCategory = document.querySelector('input[name="category"]');
const inputBskyUri = document.querySelector('input[name="bsky_uri"]');
const editor = document.querySelector("#editor");
const editorPreview = document.querySelector("#editorPreview");

function preview() {
  editorPreview.innerHTML = `
    <h1 class="blog-head">${inputTitle.value}</h1>
    <span class="blog-subhead"><em>${inputSubtitle.value}</em></span>
    <p class='blog-publish'>
      🕒 ${inputDt.value}
    </p>
    <hr class='frontmatter'>
  `;
  editorPreview.innerHTML += editor.value;
}

// Enhanced editor features

const handleTabs = (el) => {
  const TAB = "    ";

  el.addEventListener("keydown", (e) => {
    const { key, shiftKey, target } = e;
    const { value, selectionStart, selectionEnd } = target;

    if (key === "Tab") {
      e.preventDefault();
      const startLine = value.lastIndexOf("\n", selectionStart - 1) + 1;
      const endLine = value.indexOf("\n", selectionEnd);
      const endIndex = endLine === -1 ? value.length : endLine;
      const lines = value.slice(startLine, endIndex).split("\n");

      if (shiftKey) {
        let removed = 0;
        const updatedLines = lines.map((line) => {
          if (line.startsWith(TAB)) {
            removed += TAB.length;
            return line.slice(TAB.length);
          }
          return line;
        });
        target.value =
          value.slice(0, startLine) +
          updatedLines.join("\n") +
          value.slice(endIndex);
        if (removed > 0) {
          target.selectionStart = Math.max(selectionStart - TAB.length, 0);
          target.selectionEnd = Math.max(
            selectionEnd - removed,
            target.selectionStart,
          );
        }
      } else {
        const updatedLines = lines.map((line) => TAB + line);
        target.value =
          value.slice(0, startLine) +
          updatedLines.join("\n") +
          value.slice(endIndex);
        const shiftAmount = lines.length * TAB.length;
        target.selectionStart = selectionStart + TAB.length;
        target.selectionEnd = selectionEnd + shiftAmount;
      }

      preview?.();
      Prism?.highlightAll?.();
    }
  });
};

const handleEnterIndent = (el) => {
  el.addEventListener("keydown", (e) => {
    if (e.key !== "Enter") return;
    e.preventDefault();

    const { value, selectionStart, selectionEnd } = el;
    const before = value.slice(0, selectionStart);
    const after = value.slice(selectionEnd);
    const lineStart = before.lastIndexOf("\n") + 1;
    const currentLine = before.slice(lineStart);
    const match = currentLine.match(/^\s*/);
    const indent = match ? match[0] : "";

    el.value = before + "\n" + indent + after;
    const caretPos = selectionStart + 1 + indent.length;
    el.selectionStart = el.selectionEnd = caretPos;

    preview?.();
    Prism?.highlightAll?.();
  });
};

const handleBackspace = (el) => {
  const TAB = "    ";
  el.addEventListener("keydown", (e) => {
    if (e.key !== "Backspace") return;
    const { value, selectionStart, selectionEnd } = el;
    const prevFour = value.slice(selectionStart - 4, selectionStart);
    if (prevFour === TAB) {
      e.preventDefault();
      el.value = value.slice(0, selectionStart - 4) + value.slice(selectionEnd);
      el.selectionStart = el.selectionEnd = selectionStart - 4;

      preview?.();
      Prism?.highlightAll?.();
    }
  });
};

const VOID_ELEMENTS = new Set([
  "area",
  "base",
  "br",
  "col",
  "embed",
  "hr",
  "img",
  "input",
  "link",
  "meta",
  "param",
  "source",
  "track",
  "wbr",
]);

const handleAutoClose = (el) => {
  el.addEventListener("input", () => {
    const { value, selectionStart, selectionEnd } = el;
    if (value[selectionStart - 1] !== ">") return;

    const beforeCaret = value.slice(0, selectionStart);
    const match = beforeCaret.match(/<([a-zA-Z0-9-]+)(\s[^<>]*)?>$/);
    if (!match) return;

    const tagName = match[1].toLowerCase();
    if (VOID_ELEMENTS.has(tagName)) return;

    const afterCaret = value.slice(selectionEnd);
    const closingTag = `</${tagName}>`;

    // Check if the closing tag already exists right after the caret
    if (
      value.slice(selectionStart, selectionStart + closingTag.length) ===
      closingTag
    ) {
      return; // don’t insert a duplicate
    }

    if (selectionStart === selectionEnd) {
      // single-line auto-close
      el.value = beforeCaret + closingTag + afterCaret;
      el.selectionStart = el.selectionEnd = selectionStart;
    }

    preview?.();
    Prism?.highlightAll?.();
  });
};

const enableEditorFeatures = (el) => {
  if (!el) return;
  el.value = el.value.replace(/\t/g, "    ");

  handleTabs(el);
  handleEnterIndent(el);
  handleBackspace(el);
  handleAutoClose(el);
};

// SpeechBox web component
class SpeechBoxElement extends HTMLElement {
  connectedCallback() {
    const char = this.getAttribute("character");
    const emotion = this.getAttribute("emotion");

    const images = {
      deer: {
        neutral: {
          src: "/static/speech/deer/neutral.png",
          alt: "drawing of a deer, talking to you.",
        },
        happy: {
          src: "/static/speech/deer/happy.png",
          alt: "drawing of a happy deer.",
        },
        shocked: {
          src: "/static/speech/deer/shock.png",
          alt: "drawing of a shocked deer.",
        },
        worried: {
          src: "/static/speech/deer/sad.png",
          alt: "drawing of a sad or worried deer.",
        },
      },
      you: { src: "/static/speech/you.png", alt: "drawing of you, smiling." },
    };

    const { src, alt } =
      images[char]?.[emotion] || images[char]?.neutral || images.deer;

    this.innerHTML = `
      <div class="dialog-box">
        <img width="120" height="120" class="raw dialog profile" src="${src}" alt="${alt}">
        <div class="dialog speech ${char}">
          ${this.innerHTML}
        </div>
      </div>`;
  }
}

customElements.define("speech-box", SpeechBoxElement);

async function syncDraft() {
  await fetch(`/blog/new/sync`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      title: inputTitle.value,
      slug: inputSlug.value,
      contents: editor.value,
      creation_datetime: inputDt.value,
      subtitle: inputSubtitle.value || null,
      category: inputCategory.value || null,
      bsky_uri: inputBskyUri.value || null,
    }),
  });
}

function updatePreview(sync = false) {
  preview();
  Prism.highlightAll();
  if (typeof createFootnotes === "function") createFootnotes();
  if (sync) syncDraft();
}

// --- Entry point ---
function initPostEditor(mode = "edit") {
  enableEditorFeatures(document.querySelector("#editor"));

  const sync = mode === "new";

  [editor, inputTitle, inputSubtitle, inputSlug, inputDt].forEach((el) =>
    el?.addEventListener("input", () => updatePreview(sync)),
  );

  preview();
}

// Export the initializer
export { initPostEditor };
