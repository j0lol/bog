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

enableTab('editor');

const inputTitle = document.querySelector('input[name=\"title\"]');
const inputSlug = document.querySelector('input[name=\"slug\"]');
const inputDt = document.querySelector('input[name=\"creation_datetime\"]');
const editor = document.querySelector('#editor');
const editorPreview = document.querySelector('#editorPreview');

function preview() {
  editorPreview.innerHTML = `
    <h1>${inputTitle.value}</h1>
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
      creation_datetime: inputDt.value
    }),
  });
}

[editor, inputTitle, inputSlug, inputDt].forEach((el) => {
  el.addEventListener('input', (event) => {
    preview();
    syncDraft();
  });
})

preview();
