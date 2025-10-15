async function login() {
  const pass = prompt("What's the code?");

  if (pass != null) {
    const resp = await fetch(`/login`, {
      method: "POST",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
      },
      body: pass,
    });

    const txt = await resp.text();
    console.log(txt);
  }

}

document.querySelector("#login-button")
  .addEventListener('click', (e) => { login() });
