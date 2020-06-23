import { validateEmail, validateDate } from "verus-wasm";

const emailInput = document.getElementById("email-input");
const emailWasm = document.getElementById("email-wasm");
const emailRuby = document.getElementById("email-ruby");

const dateInput = document.getElementById("date-input");
const dateWasm = document.getElementById("date-wasm");
const dateRuby = document.getElementById("date-ruby");

const validityText = input => (input ? "Valid" : "Invalid");

const emailWasmListener = () =>
  (emailWasm.innerText = validityText(validateEmail(emailInput.value)));
const dateWasmListener = () =>
  (dateWasm.innerText = validityText(validateDate(dateInput.value)));

const emailRubyListener = async () => {
  const isValid = await (await fetch("http://localhost:4567/validate-email", {
    method: "POST",
    mode: "cors",
    body: JSON.stringify(emailInput.value),
  })).json();

  emailRuby.innerText = validityText(isValid);
};

const dateRubyListener = async () => {
  const isValid = await (await fetch("http://localhost:4567/validate-date", {
    method: "POST",
    mode: "cors",
    body: JSON.stringify(dateInput.value),
  })).json();

  dateRuby.innerText = validityText(isValid);
};

emailInput.onkeyup = () => {
  emailWasmListener();
  emailRubyListener();
};

dateInput.onkeyup = () => {
  dateWasmListener();
  dateRubyListener();
};
