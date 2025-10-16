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
            src = "/static/speech/deer/sad.png"
            alt = "drawing of a sad or worried deer, talking to you."
            break;
          case "happy":
            src = "/static/speech/deer/happy.png"
            alt = "drawing of a happy deer, talking to you."
            break;
          case "shocked":
            src = "/static/speech/deer/shock.png"
            alt = "drawing of a shocked deer, talking to you." 
            break;

          case "neutral":
          default:
            src = "/static/speech/deer/neutral.png"
            alt = "drawing of a deer, talking to you."
            break;
        }
        break;
      case "you":
        src = "/static/speech/you.png"
        alt = "drawing of you, smiling"
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
