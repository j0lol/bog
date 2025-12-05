if (
  document.cookie
    .split("; ")
    .find((row) => row.startsWith("HideJobCallout="))
    ?.split("=")[1] == "true"
) {
  document.head.appendChild(document.createElement("style")).textContent =
    "#jobCallout { display: none }";
}
