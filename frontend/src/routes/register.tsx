import { createStore } from "solid-js/store";
import { createEffect } from "solid-js";

type FormFields = {
  role_id: Number;
  name: string;
  username: string;
  password: string;
  dob: string;
  email: string;
  phone_no: string;
  location: string;
  languages: string;
  about: string;
  profilepicurl: string;
  bannerurl: string;
};

type RegistrationFields = Pick<
  FormFields,
  "role_id" | "name" | "username" | "password" | "dob" | "email"
>;

type ProfileResponseFields = Pick<
  FormFields,
  | "username"
  | "phone_no"
  | "location"
  | "languages"
  | "about"
  | "profilepicurl"
  | "bannerurl"
>;
type LoginResponseFields = Pick<
  FormFields,
  "username" | "role_id" | "password"
>;
/*
curl -X POST \
-H "Content-Type: application/json" \
-d '{"role_id":1,"name":"Pradyumna Malladi","username":"mssrprad","password":"password123","dob":"2003-01-13","email":"f20210367@hyderabad.bits-pilani.ac.in"}' \
127.0.0.1:8000/register
*/

const submit = async (form: FormFields) => {
  // should be submitting your form to some backend service
  try {
    const registrationData: RegistrationFields = {
      role_id: form.role_id,
      name: form.name,
      username: form.username,
      password: form.password,
      dob: form.dob,
      email: form.email,
    };
    const ProfileData: ProfileResponseFields = {
      username: form.username,
      phone_no: form.phone_no,
      location: form.location,
      languages: form.languages,
      about: form.about,
      profilepicurl: form.profilepicurl,
      bannerurl: form.bannerurl,
    };
    const LoginData: LoginResponseFields = {
      username: form.username,
      password: form.password,
      role_id: form.role_id,
    };
    var data = JSON.stringify(registrationData);
    console.log(data);
    var prof_data = JSON.stringify(ProfileData);
    console.log(prof_data);
    var login_data = JSON.stringify(LoginData);
    console.log(login_data);
    const response = await fetch("http://127.0.0.1:8000/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: data,
    });
    console.log(response);

    if (response.ok) {
      // Request was successful
      const data = await response.json();
      console.log(data);
      // Process the response data here
      console.log("succeeded in registering!");

      const login_response = await fetch("http://localhost:8000/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: login_data,
        credentials: "include", // Don't forget to specify this if you need cookies
      });
      console.log(login_response);
      if (login_response.ok) {
        const prof_response = await fetch("http://localhost:8000/profile/me", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: prof_data,
          credentials: "include",
        });
        if (prof_response.ok) {
          console.log("succeeded in saving profile!");
        } else {
          console.log("failed in saving profile!");
        }
      } else {
        console.log("failed in logging in!");
      }

      console.log(response);
    } else {
      // Handle error response
      const errorData = await response.json();
      console.log(errorData);
      // Handle error data here
      console.log("failed in submitting!");
    }
  } catch (error) {
    // Handle network or other errors
    console.error("Error:", error);
    console.log("failed in submitting!");
  }
  console.log(`submitting ${JSON.stringify(form)}`);
};

const useForm = () => {
  const [form, setForm] = createStore<FormFields>({
    role_id: 1,
    name: "",
    username: "",
    password: "",
    dob: "",
    email: "",
    phone_no: "",
    location: "",
    languages: "english",
    about: "Someone",
    profilepicurl:
      "https://cdn.pixabay.com/photo/2015/10/05/22/37/blank-profile-picture-973460_1280.png",
    bannerurl:
      "https://images.pexels.com/photos/573130/pexels-photo-573130.jpeg?cs=srgb&dl=pexels-zulian-yuliansyah-573130.jpg&fm=jpg",
  });

  const clearField = (fieldName: string) => {
    setForm({
      [fieldName]: "",
    });
  };

  const updateFormField = (fieldName: string) => (event: Event) => {
    const inputElement = event.currentTarget as HTMLInputElement;
    if (inputElement.type === "checkbox") {
      setForm({
        [fieldName]: !!inputElement.checked,
      });
    } else {
      setForm({
        [fieldName]: inputElement.value,
      });
    }
  };

  return { form, submit, updateFormField, clearField };
};

// To Do: Implement Validation for languages
const validateLanguages = (value: string) => {
  const languagesRegex = /^[a-zA-Z]+(,[a-zA-Z]+)*$/; // Regular expression to validate languages separated by commas
  if (!value.match(languagesRegex)) {
    return "Enter languages in the format 'english,telugu,hindi'";
  }
  return null; // Return null if validation passes
};

export default function Register() {
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
      <h1 class="text-4xl font-bold">Register</h1>
      <div id="form" class="flex flex-auto justify-center p-10">
        <form class="w-full max-w-lg p-20 bg-white shadow-2xl rounded">
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full md:w-1/2 px-3 mb-6 md:mb-0">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="grid-fullname"
              >
                Full Name
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-red-500 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white"
                id="grid-fullname"
                type="text"
                placeholder="Jane"
                maxLength={255}
                value={form.name}
                onChange={updateFormField("name")}
              />
            </div>
            <div class="w-full md:w-1/2 px-3">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="grid-username"
              >
                username
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="grid-username"
                type="text"
                placeholder="Doe"
                maxLength={20}
                value={form.username}
                onChange={updateFormField("username")}
              />
            </div>
          </div>
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full px-3">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="grid-password"
              >
                Password
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="grid-password"
                type="password"
                placeholder="******************"
                value={form.password}
                onChange={updateFormField("password")}
              />
              <p class="text-gray-600 text-xs italic">
                Make it as long and as crazy as you'd like
              </p>
            </div>
          </div>
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full px-3">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="grid-password"
              >
                Your Date of Birth
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="grid-dob"
                type="date"
                placeholder="01/01/2000"
                value={form.dob}
                onChange={updateFormField("dob")}
              />
            </div>
          </div>
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full px-3">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="grid-email"
              >
                Email
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="grid-email"
                type="email"
                placeholder="foo@foo.com"
                maxLength={255}
                value={form.email}
                onChange={updateFormField("email")}
              />
              <p class="text-gray-600 text-xs italic">
                Enter a valid email please.
              </p>
            </div>
          </div>
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full px-3">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="grid-email"
              >
                Phone No
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="grid-phone"
                type="tel"
                placeholder="1111111111"
                maxLength={10}
                minLength={10}
                value={form.phone_no}
                onChange={updateFormField("phone_no")}
              />
              <p class="text-gray-600 text-xs italic">
                Enter a valid Phone No please.
              </p>
            </div>
          </div>
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="px-3 w-full">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="grid-languages"
              >
                Languages you understand:
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="grid-languages"
                type="text"
                placeholder="english"
                maxLength="500"
                value={form.languages}
                onChange={updateFormField("languages")}
              />
              <p class="text-gray-600 text-xs italic">
                Enter languages with spaces.
              </p>
            </div>
          </div>
          <div class="flex flex-wrap justify-center -mx-3 mb-6">
            <div class="w-full md:w-2/3 px-3 mb-6 md:mb-0">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="grid-location"
              >
                Your location (in 20 chars)
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="grid-location"
                type="text"
                placeholder="Terra"
                maxlength="20"
                value={form.location}
                onChange={updateFormField("location")}
              />
            </div>
          </div>
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="grid-languages"
              >
                Something about yourself for others!
              </label>
              <textarea
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="grid-about"
                placeholder="nothing much!"
                maxLength="500"
                value={form.about}
                onChange={updateFormField("about")}
              />
            </div>
          </div>
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="grid-profilepicurl"
              >
                Link an image for your profile picture!
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="grid-profilepicurl"
                placeholder=""
                maxLength="500"
                value={form.profilepicurl}
                onChange={updateFormField("profilepicurl")}
              />
            </div>
          </div>
          <div class="flex flex-wrap -mx-3 mb-6">
            <div class="w-full">
              <label
                class="block uppercase tracking-wide text-gray-700 text-xs font-bold mb-2"
                for="grid-bannerurl"
              >
                Link an image for your banner picture!
              </label>
              <input
                class="appearance-none block w-full bg-gray-200 text-gray-700 border border-gray-200 rounded py-3 px-4 mb-3 leading-tight focus:outline-none focus:bg-white focus:border-gray-500"
                id="grid-bannerurl"
                placeholder=""
                maxLength="500"
                value={form.bannerurl}
                onChange={updateFormField("bannerurl")}
              />
            </div>
          </div>
          <p class="p-10 text-red-500 text-xs italic">
            All fields are compulsory.
          </p>
          <button
            class="p-10 bg-blue-500 hover:bg-orange-700 text-white font-bold py-2 px-4 rounded"
            type="submit"
            value="Submit"
            onClick={(event) => handleSubmit(event)}
          >
            Button
          </button>
        </form>
      </div>
    </main>
  );
}