import { Setter, createSignal, onCleanup } from "solid-js";
import { VsLocation } from "solid-icons/vs";
import { IoLanguageSharp } from "solid-icons/io";
import { createEffect } from 'solid-js';


type UserProfileProps = {
  username: string;
  is_followed: boolean;
  follows: boolean;
  no_of_followers: number;
  no_of_following: number;
  addUrl: Setter<string>;
};

const UserProfile = (props: UserProfileProps) => {
  console.log("props");
  console.log(props);
  const username = props.username;
  console.log("username: " + username);
  const [userProfile, setUserProfile] = createSignal({
    about: "",
    created_at: "",
    dob: "",
    languages: "",
    location: "",
    name: "",
    phone_no: "",
    username: "",
    profilepicurl: "",
    bannerurl: "",
  });

  const handleFollow = () => {
    // Send the GET request to /follow/{username}
    fetch('http://localhost:8000/follow/' + username, {
      method: 'GET',
      credentials: "include",
    })
      .then(response => {
        // Handle the response
        if (response.ok) {
          // Success, do something
          console.log('following succeeded');
        } else {
          // Error, handle accordingly
          console.log('following failed');
        }
      })
      .catch(error => {
        // Handle the error
      });
  };

  onCleanup(() => {
    // Cleanup any subscriptions or resources
  });

  createEffect(() => {
    // Cleanup the effect
    return () => {
      // Cleanup code
    };
  });

  // Fetch the user profile data
  const fetchUserProfile = async () => {
    try {
      const response = await fetch(
        "http://localhost:8000/profile/" + username,
        {
          method: "GET",
          headers: {
            "Content-Type": "application/json",
          },
          credentials: "include",
        }
      );
      const data = await response.json()!; // Extract JSON data from the response
      setUserProfile((c) => data); // Set the user profile data
    } catch (error) {
      console.error("Error fetching user profile:", error);
    }
    props.addUrl(userProfile().profilepicurl);
  };
  
  fetchUserProfile(); // Fetch the user profile data on component mount
  const profile = JSON.stringify(userProfile);
  console.log("profile");
  console.log(profile);
  return (
    <div>
      {userProfile() ? (
        <div class="">
          {/* <!-- User card--> */}
          <div>
            <img
              class="border-solid border-black border-2 w-full bg-cover bg-no-repeat bg-center"
              style="height: 200px;"
              src={userProfile().bannerurl}
            >
              {/* <img class="opacity-0 w-full h-full" src={ userProfile().bannerurl } alt="" /> */}
            </img>
            <div class="p-4">
              <div class="relative flex w-full">
                {/* <!-- Avatar --> */}
                <div class="flex flex-1">
                  <div style="margin-top: -6rem;">
                    <div
                      style="height:9rem; width:9rem;"
                      class="md rounded-full relative avatar"
                    >
                      <img
                        style="height:9rem; width:9rem;"
                        class="md rounded-full relative border-4 border-gray-900"
                        src={userProfile().profilepicurl}
                        alt=""
                      />
                      <div class="absolute"></div>
                    </div>
                  </div>
                </div>
                {/* <!-- Follow Button --> */}
                <div class="flex flex-col text-right">
                  {props.follows ? (
                    <button class="flex justify-center bg-blue-400 max-h-max whitespace-nowrap focus:outline-none  focus:ring rounded max-w-max border bg-transparent border-blue-500 text-blue-500 hover:border-blue-800 hover:border-blue-800 flex items-center hover:shadow-lg font-bold py-2 px-4 rounded-full mr-0 ml-auto">
                      Following
                    </button>
                  ) : (
                    <button onClick={handleFollow} class="flex justify-center bg-stone-200 max-h-max whitespace-nowrap focus:outline-none  focus:ring rounded max-w-max border bg-transparent border-blue-500 text-blue-500 hover:border-blue-800 hover:border-blue-800 flex items-center hover:shadow-lg font-bold py-2 px-4 rounded-full mr-0 ml-auto">
                      Follow
                    </button>
                  )}

                </div>
              </div>

              {/* <!-- Profile info --> */}
              <div class="space-y-1 justify-center w-full mt-3 ml-3">
                {/* <!-- User basic--> */}
                <div>
                  <p class="font-bold text-white text-xl">{userProfile().name}</p>
                  {/* <h2 class="text-xl leading-6 font-bold text-white">{userProfile()?.languages}</h2> */}
                  <p class="text-md leading-5 font-medium text-stone-100">
                    @{userProfile().username}
                  </p>
                  {props.is_followed ? (
                    <button class="m-1 flex justify-center bg-stone-200 max-h-max whitespace-nowrap focus:outline-none  focus:ring rounded border bg-transparent border-blue-500 text-blue-500 hover:border-blue-800 hover:border-blue-800 flex hover:shadow-lg font-bold py-2 px-4 rounded-full ml-0">
                      <div class="text-blue-500 text-sm p-0">Follows you</div>
                    </button>
                  ) : (
                    <div></div>
                  )}
                </div>
                {/* <!-- Description and others --> */}
                <div class="mt-3">
                  <p class="text-stone-200 leading-tight mb-2">
                    {userProfile().about}
                  </p>
                  <div class="text-stone-300 flex">
                    <span class="flex mr-2">
                      <IoLanguageSharp class="text-white" />
                      <span class="leading-5 ml-1">
                        {userProfile().languages}
                      </span>
                    </span>
                    <span class="flex mr-2">
                      <svg viewBox="0 0 24 24" class="h-5 w-5 paint-icon">
                        <g>
                          <path d="M19.708 2H4.292C3.028 2 2 3.028 2 4.292v15.416C2 20.972 3.028 22 4.292 22h15.416C20.972 22 22 20.972 22 19.708V4.292C22 3.028 20.972 2 19.708 2zm.792 17.708c0 .437-.355.792-.792.792H4.292c-.437 0-.792-.355-.792-.792V6.418c0-.437.354-.79.79-.792h15.42c.436 0 .79.355.79.79V19.71z"></path>
                          <circle cx="7.032" cy="8.75" r="1.285"></circle>
                          <circle cx="7.032" cy="13.156" r="1.285"></circle>
                          <circle cx="16.968" cy="8.75" r="1.285"></circle>
                          <circle cx="16.968" cy="13.156" r="1.285"></circle>
                          <circle cx="12" cy="8.75" r="1.285"></circle>
                          <circle cx="12" cy="13.156" r="1.285"></circle>
                          <circle cx="7.032" cy="17.486" r="1.285"></circle>
                          <circle cx="12" cy="17.486" r="1.285"></circle>
                        </g>
                      </svg>{" "}
                      <span class="leading-5 ml-1">
                        Joined: {userProfile().created_at}
                      </span>
                    </span>
                    <span class="flex mr-2">
                      <VsLocation />
                      <span class="leading-5 ml-1">
                        {userProfile().location}
                      </span>
                    </span>
                  </div>
                </div>
                <div class="pt-3 flex justify-start items-start w-full divide-x divide-gray-800 divide-solid">
                  <div class="text-center pr-3">
                    <span class="font-bold text-white">{props.no_of_following}</span>
                    <span class="text-stone-200"> Following</span>
                  </div>
                  <div class="text-center px-3">
                    <span class="font-bold text-white"> {props.no_of_followers} </span>
                    <span class="text-stone-200"> Followers</span>
                  </div>
                </div>
              </div>
            </div>
            <hr class="border-gray-800" />
          </div>
        </div>
      ) : (
        <p>Loading user profile...</p>
      )}
      <div></div>
    </div>
  );
};

export default UserProfile;