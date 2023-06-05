import { Accessor } from "solid-js";
import { createStore } from "solid-js/store";
import { Component, createEffect } from "solid-js";

type FormFields = {
    username: String;
    phone_no: String;
    location: String;
    languages: String;
    about: String;
};

/*
curl -X POST \
-H "Content-Type: application/json" \
-d '{"role_id":1,"name":"Pradyumna Malladi","username":"mssrprad","password":"password123","dob":"2003-01-13","email":"f20210367@hyderabad.bits-pilani.ac.in"}' \
127.0.0.1:8000/register
*/

const submit = async (form: FormFields) => {
  // should be submitting your form to some backend service
  try {
    var data = JSON.stringify(form);
    console.log(data);
    const response = await fetch('http://127.0.0.1:8000/profile/me', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: data,
    });
    console.log(response);

    if (response.ok) {
      // Request was successful
      const data = await response.json();
      console.log(data);
      // Process the response data here
      console.log("succeeded in submitting!");
    } else {
      // Handle error response
      const errorData = await response.json();
      console.log(errorData);
      // Handle error data here
      console.log("failed in submitting!");
    }
  } catch (error) {
    // Handle network or other errors
    console.error('Error:', error);
    console.log("failed in submitting!");
  }
  console.log(`submitting ${JSON.stringify(form)}`);
};

const useForm = () => {
  const [form, setForm] = createStore<FormFields>({
    username: "",
    phone_no: "",
    location: "",
    languages: "",
    about: "",
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

export default function CreateOrUpdateProfile() {

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
      <h1 class="text-4xl font-bold">Create/Update Profile</h1>
      <div id="form" class = "flex flex-auto justify-center p-10">
            <form class="w-full max-w-lg p-20 bg-stone-100 rounded-lg">
        <div class="flex flex-wrap -mx-3 mb-6">
          <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
            <label class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" for="grid-fullname">
              Full Name
            </label>
            <input class="appearance-none block w-full bg-gray-200 text-gray-700 border border-red-500 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white" id="grid-fullname" type="text" placeholder="Jane"
            value={form.name}
            onChange={updateFormField("name")}/>
          </div>
          <div class="w-full md:w-1/2 px-3">
            <label class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" for="grid-username">
              username
            </label>
            <input class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-username" type="text" placeholder="Doe" 
            value={form.username}
            onChange={updateFormField("username")}/>
          </div>
        </div>
        <div class="flex flex-wrap -mx-3 mb-6">
          <div class="w-full px-3">
            <label class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" for="grid-password">
              Password
            </label>
            <input class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-password" type="password" placeholder="******************" 
            value={form.password}
            onChange={updateFormField("password")}/>
            <p class="text-gray-600 text-xs italic">Make it as long and as crazy as you'd like</p>
          </div>
        </div>
        <div class="flex flex-wrap -mx-3 mb-6">
          <div class="w-full px-3">
            <label class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" for="grid-email">
              Email
            </label>
            <input class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-email" type="email" placeholder="foo@foo.com" 
            value={form.email}
            onChange={updateFormField("email")}/>
            <p class="text-gray-600 text-xs italic">Enter a valid email please.</p>
          </div>
        </div>
        <div class="flex flex-wrap justify-center -mx-3 mb-2">
          <div class="w-full md:w-2/3 px-3 mb-6 md:mb-0">
            <label class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2" for="grid-dob">
              Date Of Birth
            </label>
            <input class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500" id="grid-dob" type="date" placeholder="01/01/2000" 
            value={form.dob}
            onChange={updateFormField("dob")}/>
          </div>
        </div>
        <p class="p-10 text-red-500 text-xs italic">All fields are compulsory.</p>
        <button class="p-10 bg-blue-500 hover:bg-orange-700 text-white font-bold py-2 px-4 rounded"
        type="submit" value="Submit" onClick={(event) => handleSubmit(event)}>
            Button
        </button>
      </form>
      </div>
    </main>
  );
}