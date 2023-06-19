import { Accessor, createSignal } from "solid-js";
import { createStore } from "solid-js/store";
import { Component, createEffect } from "solid-js";
import { redirect, useNavigate } from "solid-start";

type FormFields = {
  username: string;
  password: string;
  role_id: number;
};

const submit = async (form: FormFields) => {
  const [cookie, setCookie] = createSignal('');
  // should be submitting your form to some backend service
  try {
    var data = JSON.stringify(form);
    console.log(data);
    const response = await fetch('http://localhost:8000/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: data,
      credentials: "include", // Don't forget to specify this if you need cookies
    });
    if (response.ok) {
      // Request was successful
      const data = await response.json();
      console.log(data);
      const cookie = await response.headers.entries();
      console.log(...cookie);
      // Process the response data here
      console.log("set-cookie:");
      console.log(response.headers.get('set-cookie'));
      console.log("succeeded in fetching!");
      console.log("cookies:");
      console.log(document.cookie);
    } else {
      // Handle error response
      const errorData = await response.json();
      console.log(errorData);
      // Handle error data here
      console.log("failed in fetching!");
    }
  } catch (error) {
    // Handle network or other errors
    console.error('Error:', error);
    console.log("failed in fetching!");
  }
  console.log(`submitting ${JSON.stringify(form)}`);

  
};

const useForm = () => {
  const [form, setForm] = createStore<FormFields>({
    username: "",
    password: "",
    role_id: 1,
  });

  const clearField = (fieldName: string) => {
    setForm({
      [fieldName]: ""
    });
  };

  const updateFormField = (fieldName: string) => (event: Event) => {
    const inputElement = event.currentTarget as HTMLInputElement;
    if (inputElement.type === "checkbox") {
      setForm({
        [fieldName]: !!inputElement.checked
      });
    } else {
      setForm({
        [fieldName]: inputElement.value
      });
    }
  };

  return { form, submit, updateFormField, clearField };
};

export default function Login() {

  const { form, updateFormField, submit, clearField } = useForm();

  const handleSubmit = (event: Event): void => {
    event.preventDefault();
    submit(form);
  };

  createEffect(() => {
    console.log(form);
  });

  return (
    <main class="text-center items-center mx-auto text-gray-700 p-4">
      <h1 class="text-4xl font-bold">Login</h1>
      <div class="p-20">
        <div class="flex flex-auto justify-center">
        <form class="bg-white shadow-2xl rounded px-8 pt-6 pb-8 mb-4">
            <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="username">
                Username
            </label>
            <input class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline" id="username" type="text" placeholder="Username" 
            value={form.username}
            onChange={updateFormField("username")}/>
            </div>
            <div class="mb-6">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="password">
                Password
            </label>
            <input class="shadow appearance-none border border-red-500 rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline" id="password" type="password" placeholder="******************" 
            value={form.password}
            onChange={updateFormField("password")}/>
            <p class="text-red-500 text-xs italic">Please choose a password.</p>
            </div>
            <div class="flex justify-center">
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
            type="submit" value="Submit" onClick={(event) => handleSubmit(event)}>
                Sign In
            </button>
            </div>
        </form>
        </div>
        <p class="text-center text-gray-500 text-xs">
            &copy;2020 Acme Corp. All rights reserved.
        </p>
        </div>
    </main>
  );
}