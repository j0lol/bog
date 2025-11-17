import { codeToHtml } from 'shiki';

function normalizeSourceCode(str) {
  const trimmed = str.trim();
  if (trimmed.startsWith("<!--") && trimmed.endsWith("-->")) {
    str = trimmed.replace(new RegExp('-->$'), "").replace(new RegExp('^<!--'), "");
  }
  return str.trim();
}
export function highlightAll() {
  document.querySelectorAll("pre > code").forEach(async (el) => {

    const lang = el.classList.values().filter((s) => s.startsWith("language-")).map((s) => s.replace("language-", "")).toArray()[0] ?? "plain";

    const code = normalizeSourceCode(el.innerHTML);

    console.log(lang);

    const html = await codeToHtml(code, {
      lang: lang,
      themes: {
        light: 'min-light',
        dark: 'catppuccin-mocha',
      }
    });

    el.parentElement.outerHTML = `<div class="code-block"><div class="code-language">${lang}</div>${html}</div>`;
  });
}
