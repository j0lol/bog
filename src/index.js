import "./login.js";

const job_callout = document.querySelector("#jobCallout");
const job_button = document.querySelector("#jobCalloutHide");
const job_button_forever = document.querySelector("#jobCalloutHideForever");

job_button?.addEventListener("click", (e) => {
  job_callout?.remove();

  /* Lasts for browser session */
  document.cookie =
    "HideJobCallout=true; max-age=7200; SameSite=strict; Secure";
});

job_button_forever?.addEventListener("click", (e) => {
  job_callout?.remove();

  /* 10 years should outlast this notice */
  document.cookie =
    "HideJobCallout=true; max-age=315360000; SameSite=strict; Secure";
});

if (
  document.cookie
    .split("; ")
    .find((row) => row.startsWith("HideJobCallout="))
    ?.split("=")[1] == "true"
) {
  job_callout?.remove();
}
