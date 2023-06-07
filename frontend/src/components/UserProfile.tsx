import { createSignal, onCleanup } from 'solid-js';

type UserProfileProps = {
    username: string;
};

const UserProfile = (props: UserProfileProps ) => {
    
    const username = props.username;
    console.log("username: "+username);
    const [userProfile, setUserProfile] = createSignal(
        {about: "",
        created_at:"",
        dob:"",
        languages:"",
        location:"",
        name:"",
        phone_no:"",
        username:"",}
    );
  
    onCleanup(() => {
      // Cleanup any subscriptions or resources
    });
  
    // Fetch the user profile data
    const fetchUserProfile = async () => {
        try {
            const response = await fetch('http://localhost:8000/profile/' + username, {
                method: 'GET',
                headers: {
                'Content-Type': 'application/json',
                },
                credentials: 'include',
            });
            const data = await response.json()!; // Extract JSON data from the response
            setUserProfile(c => data); // Set the user profile data
        } catch (error) {
            console.error('Error fetching user profile:', error);
        }
    };
  
    fetchUserProfile(); // Fetch the user profile data on component mount
    const profile = JSON.stringify(userProfile);
    console.log("profile");
    console.log(profile);
    return (
      <div>
        {userProfile() ? (
          <div>
            <h1>Name: {userProfile()?.name}</h1>
            <h1>Username: {userProfile()?.username}</h1>
            <p>dob: {userProfile()?.dob}</p>
            <p>Languages: {userProfile()?.languages}</p>
            <p>About: {userProfile()?.about}</p>
            <p>Created At: {userProfile()?.created_at}</p>
            <p>Phone: {userProfile()?.phone_no}</p>
            <p>Location: {userProfile()?.location}</p>
          </div>
        ) : (
          <p>Loading user profile...</p>
        )}
      </div>
    );
  };
  
  export default UserProfile;