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

function updatePreview() {
  preview();
  Prism.highlightAll();
  createFootnotes();
}

[editor, inputTitle, inputSubtitle, inputSlug, inputDt].forEach((el) => {
  el.addEventListener("input", (event) => {
    updatePreview();
  });
});

preview();

export { updatePreview };
