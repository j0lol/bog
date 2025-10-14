const icon_likes = `<span aria-label="Likes"><svg
  style="height: 1em; width: 1em; vertical-align: top;"
  viewBox="0 0 24 24"
  fill="none"
  xmlns="http://www.w3.org/2000/svg"
>
  <path
    fill-rule="evenodd"
    clip-rule="evenodd"
    d="M12.0122 5.57169L10.9252 4.48469C8.77734 2.33681 5.29493 2.33681 3.14705 4.48469C0.999162 6.63258 0.999162 10.115 3.14705 12.2629L11.9859 21.1017L11.9877 21.0999L12.014 21.1262L20.8528 12.2874C23.0007 10.1395 23.0007 6.65711 20.8528 4.50923C18.705 2.36134 15.2226 2.36134 13.0747 4.50923L12.0122 5.57169ZM11.9877 18.2715L16.9239 13.3352L18.3747 11.9342L18.3762 11.9356L19.4386 10.8732C20.8055 9.50635 20.8055 7.29028 19.4386 5.92344C18.0718 4.55661 15.8557 4.55661 14.4889 5.92344L12.0133 8.39904L12.006 8.3918L12.005 8.39287L9.51101 5.89891C8.14417 4.53207 5.92809 4.53207 4.56126 5.89891C3.19442 7.26574 3.19442 9.48182 4.56126 10.8487L7.10068 13.3881L7.10248 13.3863L11.9877 18.2715Z"
    fill="currentColor"
  />
</svg></span>`;

const icon_replies = `<span aria-label="Replies"><svg
    style="height: 1em; width: 1em; vertical-align: top;"
    viewBox="0 0 24 24"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path d="M17 9H7V7H17V9Z" fill="currentColor" />
    <path d="M7 13H17V11H7V13Z" fill="currentColor" />
    <path
      fill-rule="evenodd"
      clip-rule="evenodd"
      d="M2 18V2H22V18H16V22H14C11.7909 22 10 20.2091 10 18H2ZM12 16V18C12 19.1046 12.8954 20 14 20V16H20V4H4V16H12Z"
      fill="currentColor"
    />
  </svg></span>`;

const icon_reposts = `<span aria-label="Reposts"><svg
  style="height: 1em; width: 1em; vertical-align: top;"
  viewBox="0 0 24 24"
  viewBox="0 0 24 24"
  fill="none"
  xmlns="http://www.w3.org/2000/svg"
>
  <path
    d="M18.3701 7.99993L13.8701 10.598V8.99993H6.88989V12.9999H4.88989V6.99993H13.8701V5.40186L18.3701 7.99993Z"
    fill="currentColor"
  />
  <path
    d="M10.1299 16.9999H19.1101V10.9999H17.1101V14.9999H10.1299V13.4019L5.62988 15.9999L10.1299 18.598V16.9999Z"
    fill="currentColor"
  />
</svg></span>`;

const icon_external = `
  <span aria-label="Go to external website"><svg
    style="height: 1em; width: 1em; vertical-align: top;"
    viewBox="0 0 24 24"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      d="M15.6396 7.02527H12.0181V5.02527H19.0181V12.0253H17.0181V8.47528L12.1042 13.3892L10.6899 11.975L15.6396 7.02527Z"
      fill="currentColor"
    />
    <path
      d="M10.9819 6.97473H4.98193V18.9747H16.9819V12.9747H14.9819V16.9747H6.98193V8.97473H10.9819V6.97473Z"
      fill="currentColor"
    />
  </svg></span>`;

class BskyComments extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.visibleCount = 3;
    this.thread = null;
    this.error = null;
  }

  connectedCallback() {
    const postUri = this.getAttribute("post");
    if (!postUri) {
      this.renderError("Post URI is required");
      return;
    }
    this.loadThread(postUri);
  }

  async loadThread(uri) {
    try {
      const thread = await this.fetchThread(uri);
      this.thread = thread;
      this.render();
    } catch (err) {
      console.log(err);
      this.renderError("Error loading comments");
    }
  }

  async fetchThread(uri) {
    if (!uri || typeof uri !== "string") {
      throw new Error("Invalid URI: A valid string URI is required.");
    }

    const params = new URLSearchParams({ uri });
    const url = `https://public.api.bsky.app/xrpc/app.bsky.feed.getPostThread?${params.toString()}`;
    console.log(url);

    try {
      const response = await fetch(url, {
        method: "GET",
        headers: {
          Accept: "application/json",
        },
        cache: "no-store",
      });

      if (!response.ok) {
        const errorText = await response.text();
        console.error("Fetch Error: ", errorText);
        throw new Error(`Failed to fetch thread: ${response.statusText}`);
      }

      const data = await response.json();

      if (!data.thread || !data.thread.replies) {
        throw new Error("Invalid thread data: Missing expected properties.");
      }

      return data.thread;
    } catch (error) {
      console.error("Error fetching thread:", error.message);
      throw error;
    }
  }

  render() {
    if (!this.thread || !this.thread.replies) {
      this.renderError("No comments found");
      return;
    }

    const sortedReplies = this.thread.replies.sort(
      (a, b) => (b.post.likeCount ?? 0) - (a.post.likeCount ?? 0),
    );

    const container = document.createElement("div");
    container.innerHTML = `
      <comments>
        <p class="reply-info">
          Reply on Bluesky
          <a href="https://bsky.app/profile/${this.thread.post?.author?.did}/post/${this.thread.post?.uri.split("/").pop()}" target="_blank" rel="noopener noreferrer">here</a> to join the conversation.
        </p>
        <div id="comments"></div>
        <button id="show-more">
          Show more comments
        </button>
      </comments>
    `;

    const hiddenReplies = this.thread.post.threadgate?.record.hiddenReplies;
    const commentsContainer = container.querySelector("#comments");
    sortedReplies.slice(0, this.visibleCount).forEach((reply) => {
      if (!hiddenReplies?.includes(reply.post.uri)) {
        commentsContainer.appendChild(this.createCommentElement(reply));
      }
    });
    if (commentsContainer.children.length == 0) {
      commentsContainer.remove();
    }

    const showMoreButton = container.querySelector("#show-more");
    if (this.visibleCount >= sortedReplies.length) {
      showMoreButton.remove();
    }
    showMoreButton.addEventListener("click", () => {
      this.visibleCount += 5;
      this.render();
    });

    this.shadowRoot.innerHTML = "";
    this.shadowRoot.appendChild(container);

    if (!this.hasAttribute("no-css")) {
      this.addStyles();
    }
  }

  escapeHTML(htmlString) {
    return htmlString
      .replace(/&/g, "&amp;") // Escape &
      .replace(/</g, "&lt;") // Escape <
      .replace(/>/g, "&gt;") // Escape >
      .replace(/"/g, "&quot;") // Escape "
      .replace(/'/g, "&#039;"); // Escape '
  }

  createCommentElement(reply) {
    const comment = document.createElement("div");
    comment.classList.add("comment");

    const author = reply.post.author;
    const text = reply.post.record?.text || "";

    console.log(reply);
    comment.innerHTML = `
      <div class="author">

          <a class="noline" href="https://bsky.app/profile/${author.did}">
          ${author.avatar ? `<div class="imgcontainer"><img width="22" src="${author.avatar}" /><div class="imgborder"></div></div>` : ""}
          <span><b class="author-nickname">${author.displayName ?? author.handle}‭</b> @${author.handle}</span>
          </a>
        <p class="comment-text">${this.escapeHTML(text)}</p>
        <a class="noline" href="https://bsky.app/profile/${reply.post.author?.did}/post/${reply.post.uri.split("/").pop()}"><small class="comment-meta">
        ${icon_likes} ${reply.post.likeCount ?? 0} &nbsp; ${icon_reposts} ${reply.post.repostCount ?? 0} &nbsp; ${icon_replies} ${reply.post.replyCount ?? 0} &nbsp; ${icon_external}
        </small></a>
      </div>
    `;

    if (reply.replies && reply.replies.length > 0) {
      const repliesContainer = document.createElement("div");
      repliesContainer.classList.add("replies-container");

      reply.replies
        .sort((a, b) => (b.post.likeCount ?? 0) - (a.post.likeCount ?? 0))
        .forEach((childReply) => {
          repliesContainer.appendChild(this.createCommentElement(childReply));
        });

      comment.appendChild(repliesContainer);
    }

    return comment;
  }

  renderError(message) {
    this.shadowRoot.innerHTML = `<p class="error">${message}</p>`;
  }

  addStyles() {
    const style = document.createElement("style");
    style.textContent = `
      :host {
        --background-color: white;
        --text-color: black;
        --link-color: gray;
        --link-hover-color: black;
        --comment-meta-color: gray;
        --error-color: red;
        --reply-border-color: #ccc;
        --button-background-color: rgba(0,0,0,0.05);
        --button-hover-background-color: rgba(0,0,0,0.1);
        --author-avatar-border-radius: 100%;
      }

      a.noline {
        text-decoration: none;
      }
      comments {
        margin: 0 auto;
        padding: 0;
        max-width: 720px;
        display: block;
        background-color: var(--background-color);
        color: var(--text-color);
      }

      .reply-info {
        font-size: 14px;
        color: var(--text-color);
        margin-top: 0;
      }

      .reply-info:only-child {
        margin-bottom: 0;
      }

      /* Selecting the last comment if there is no show-more button */
      #comments:not(:has(+ #show-more)) > .comment:last-child .author .comment-meta {
        margin-bottom: 0;
      }

      #show-more {
        margin-top: 10px;
        width: 100%;
        padding: 1em;
        font: inherit;
        box-sizing: border-box;
        background: var(--button-background-color);
        border-radius: 0.8em;
        cursor: pointer;
        border: 0;

        &:hover {
          background: var(--button-hover-background-color);
        }
      }
      .comment:not(:last-child) {
        margin-bottom: 2em;
      }
      .author {
        a:first-child {
          font-size: 0.9em;
          margin-bottom: 0.4em;
          display: inline-block;
          color: var(--link-color);

          &:not(:hover) {
            text-decoration: none;
          }

          .author-nickname {
            display: table-cell; /* Yeah, whatever. */
            max-width: 15ch;
            text-overflow: ellipsis;
            white-space: nowrap;
          overflow: hidden;
          }

          .author-nickname:hover {
            text-decoration: underline;
          }

          img {
            border-radius: var(--author-avatar-border-radius);
            vertical-align: middle;
            width: 24px;
            height: 24px;
            border-radius: 12px;
            background-color: rgb(241, 243, 245);
          }
          .imgcontainer {
            width: 24px;
            height: 24px;
            position: relative;
            margin-right: 0.4em;
            display: inline-block;
          }
          .imgborder  {
            position: absolute;
            inset: 0px;
            border: 1px solid rgb(212, 219, 226);
            opacity: 0.6;
            pointer-events: none;
            border-radius: 12px;
          }
        }
      }
      .comment-text {
        margin: 5px 0;
        white-space: pre-line;
      }
      .comment-meta {
        color: var(--comment-meta-color);
        display: block;
        margin: 1em 0 2em;
      }
      .replies-container, #comments > .comment {
        border-left: 0.5px solid var(--reply-border-color);
        margin-left: 0rem;
        padding-left: 0.8rem;
      }
      .error {
        color: var(--error-color);
      }

    `;
    this.shadowRoot.appendChild(style);
  }
}

customElements.define("bsky-comments", BskyComments);
