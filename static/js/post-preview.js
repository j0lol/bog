const inputTitle = document.querySelector('input[name=\"title\"]');
const inputSlug = document.querySelector('input[name=\"slug\"]');
const inputDt = document.querySelector('input[name=\"creation_datetime\"]');
const inputSubtitle = document.querySelector('input[name=\"subtitle\"]');
const inputCategory = document.querySelector('input[name=\"category\"]');
const inputBskyUri = document.querySelector('input[name=\"bsky_uri\"]');

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

// https://css-tricks.com/snippets/javascript/support-tabs-in-textareas/#comment-249547
function enableTab(id) {
  var el = document.getElementById(id);
  el.onkeydown = function (e) {
    if (e.key === "Tab") {
      // tab was pressed
      // get caret position/selection
      const val = this.value,
        start = this.selectionStart,
        end = this.selectionEnd;

      // set textarea value to: text before caret + tab + text after caret
      this.value = val.substring(0, start) + "    " + val.substring(end);

      // put caret at right position again
      this.selectionStart = this.selectionEnd = start + 4;

      // No input is sent, so we need to manually update.
      preview();
      Prism.highlightAll();

      // prevent the focus lose
      return false;
    }
    if (e.key === "Backspace") {
      // tab was pressed
      // get caret position/selection
      const val = this.value,
        start = this.selectionStart,
        end = this.selectionEnd;

      const prevTab = val.substring(start - 4, start);
      if (prevTab === "    ") {
        // the backspace is going to delete one character,
        // we need to delete 3 more...
        this.value = val.substring(0, start - 3) + val.substring(end);
        this.selectionStart = this.selectionEnd = start - 3;
        //return false;
      }
    }
  };
}

enableTab("editor");

class SpeechBoxElement extends HTMLElement {
  constructor() {
    super();
  }

  connectedCallback() {
    const char = this.getAttribute("character");
    const emotion = this.getAttribute("emotion");

    let src = "";
    let alt = "";
    switch (char) {
      case "deer":
        switch (emotion) {
          case "worried":
            src = "/static/speech/deer/sad.png";
            alt = "drawing of a sad or worried deer, talking to you.";
            break;
          case "happy":
            src = "/static/speech/deer/happy.png";
            alt = "drawing of a happy deer, talking to you.";
            break;
          case "shocked":
            src = "/static/speech/deer/shock.png";
            alt = "drawing of a shocked deer, talking to you.";
            break;

          case "neutral":
          default:
            src = "/static/speech/deer/neutral.png";
            alt = "drawing of a deer, talking to you.";
            break;
        }
        break;
      case "you":
        src = "/static/speech/you.png";
        alt = "drawing of you, smiling";
        break;
    }

    const contents = this.innerHTML;
    this.innerHTML = `
    <div class="dialog-box">
      <img width="120" height="120" class="raw dialog profile" src="${src}" alt="${alt}">
      <div class="dialog speech ${char}">
        ${contents}
      </div>
    </div>`;
  }
}

customElements.define("speech-box", SpeechBoxElement);
