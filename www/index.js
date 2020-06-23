import { validateEmail, validateDate } from "verus-wasm";

const emailInput = document.getElementById("email-input");
const emailWasm = document.getElementById("email-wasm");
const emailRuby = document.getElementById("email-ruby");

const dateInput = document.getElementById("date-input");
const dateWasm = document.getElementById("date-wasm");
const dateRuby = document.getElementById("date-ruby");

const validityText = input => (input ? "Valid" : "Invalid");

const emailWasmListener = () => emailWasm.innerText = validityText(validateEmail(emailInput.value));
const dateWasmListener = () => dateWasm.innerText = validityText(validateDate(dateInput.value));

emailInput.onkeyup = emailWasmListener;
dateInput.onkeyup = dateWasmListener;