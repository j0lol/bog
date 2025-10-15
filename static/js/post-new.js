// https://css-tricks.com/snippets/javascript/support-tabs-in-textareas/#comment-249547
function enableTab(id) {
  var el = document.getElementById(id);
  el.onkeydown = function(e) {
    if (e.keyCode === 9) { // tab was pressed
      // get caret position/selection
      var val = this.value,
        start = this.selectionStart,
        end = this.selectionEnd;

      // set textarea value to: text before caret + tab + text after caret
      this.value = val.substring(0, start) + '\t' + val.substring(end);

      // put caret at right position again
      this.selectionStart = this.selectionEnd = start + 1;

      // prevent the focus lose
      return false;
    }
  };
}

class SpeechBoxElement extends HTMLElement {
  constructor() {
    super();
  }

  connectedCallback() {
    const shadow = this.attachShadow({ mode: "open" });

    const characterClass = "foo";
    
    const dialogBox = document.createElement("div");
    dialogBox.setAttribute("class", "dialog-box");

    const speechImage = document.createElement("img");
    speechImage.setAttribute("class", "raw dialog profile");
    speechImage.setAttribute("width", "120");
    speechImage.setAttribute("height", "120");
    speechImage.setAttribute("src", "/static/speech/you.png");
    speechImage.setAttribute("alt", "poop");

    const speechContent = document.createElement("div");
    speechContent.setAttribute("class", `dialog speech ${characterClass}`);

    dialogBox.appendChild(speechImage);
    dialogBox.appendChild(speechContent);
    shadow.appendChild(dialogBox);
  }
}

customElements.define("speech-box", SpeechBoxElement);

enableTab('editor');

const inputTitle = document.querySelector('input[name=\"title\"]');
const inputSlug = document.querySelector('input[name=\"slug\"]');
const inputDt = document.querySelector('input[name=\"creation_datetime\"]');
const inputSubtitle = document.querySelector('input[name=\"subtitle\"]');
const inputCategory = document.querySelector('input[name=\"category\"]');
const inputBskyUri = document.querySelector('input[name=\"bsky_uri\"]');

const editor = document.querySelector('#editor');
const editorPreview = document.querySelector('#editorPreview');

function preview() {
  editorPreview.innerHTML = `
    <h1 class="blog-head">${inputTitle.value}</h1>
    <span class="blog-subhead"><em>${inputSubtitle.value}</em></span>
    <hr class='frontmatter'>
    <p class='blog-publish'>
      🕒 ${inputDt.value}
    </p>
  `;
  editorPreview.innerHTML += editor.value;
}

async function syncDraft() {
  const url = `/blog/new/sync`;
  const resp = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      title: inputTitle.value,
      slug: inputSlug.value,
      contents: editor.value,
      creation_datetime: inputDt.value,
      subtitle: inputSubtitle.value != "" ? inputSubtitle.value : null,
      category: inputCategory.value != "" ? inputCategory.value : null,
      bsky_uri: inputBskyUri.value != "" ? inputBskyUri.value : null,
    }),
  });
}

[editor, inputTitle, inputSubtitle, inputSlug, inputDt].forEach((el) => {
  el.addEventListener('input', (event) => {
    preview();
    syncDraft();
  });
})

preview();
