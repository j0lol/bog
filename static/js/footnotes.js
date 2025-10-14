// https://www.niquette.ca/articles/accessible-footnotes/

const style = `article {
		counter-reset: references;
}

[aria-labelledby="notes_heading"] {
		counter-increment: references;
		font-size: 0.83em;
		vertical-align: super;
		line-height: 1;
    text-decoration: none;
}
[aria-labelledby="notes_heading"]:hover {
    text-decoration: underline;
}

[aria-labelledby="notes_heading"]::after {
		content: "[" counter(references) "]";
}

footer ol {
  margin-block-end: 0;
}`;
const styleSheet = document.createElement("style");
styleSheet.textContent = style;
document.head.appendChild(styleSheet);

document.addEventListener("DOMContentLoaded", function () {
  var footerPairs = document.querySelectorAll("footer > ol > li");

  footerPairs.forEach((pair) => {
    if (!pair.id.startsWith("note_")) {
      var warn = document.createElement("span");
      warn.innerHTML = "Invalid ID. Prefix with <code>note_</code>"; // Force non-emoji on mobile (thanks Apple)
      warn.setAttribute("style", "color: red; font-weight: bold;");
      pair.appendChild(warn);
    }

    var returnLink = document.createElement("a");
    returnLink.innerHTML = "↩\ufe0e"; // Force non-emoji on mobile (thanks Apple)
    returnLink.setAttribute("aria-label", "Return to content");

    if (returnLink && pair.id) {
      returnLink.href = "#ref_" + pair.id.replace(/^note_/, "");
    }
    pair.appendChild(returnLink);
  });

  var articleAnchors = document.querySelectorAll("article a[ref]");

  articleAnchors.forEach((anchor, index) => {
    let ref = anchor.getAttribute("ref");
    anchor.removeAttribute("ref");

    var id = "ref_" + ref;
    anchor.id = id;
    anchor.href = "#note_" + ref;
    anchor.setAttribute("aria-labelledby", "notes_heading");
  });
});
