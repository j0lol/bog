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

function updatePreview() {
  preview();
  Prism.highlightAll();
  createFootnotes();
  syncDraft();
}

[editor, inputTitle, inputSubtitle, inputSlug, inputDt].forEach((el) => {
  el.addEventListener("input", (event) => {
    updatePreview();
  });
});

preview();

export { updatePreview };
