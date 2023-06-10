import { useParams } from "@solidjs/router";
import UserProfile from "~/components/UserProfile";
import { createSignal, createEffect } from 'solid-js';

type UserProps = {
  username: string;
};

type FollowDetails = {
  requesting: String;
  requested: String;
  following: boolean;
  is_followed: boolean;
};

async function fetchFollowDetails(username: string): Promise<FollowDetails | null> {
  try {
    const response = await fetch('http://localhost:8000/followdetails/'+username, {
      method: 'GET',
      credentials: "include",
    })
    if (response.ok) {
      const data = await response.json();
      return data as FollowDetails;
    } else {
      console.error('Error fetching follow details:', response.status);
      return null;
    }
  } catch (error) {
    console.error('Error fetching follow details:', error);
    return null;
  }
}

export default function User() {
  const params = useParams(); // 👈 Get the dynamic route parameters
  console.log(params);
  const username = params.username;

  const [followDetails, setFollowDetails] = createSignal({
    requesting: "",
    requested: "",
    following: false,
    is_followed: false,
  });

  createEffect(async () => {
    const data = await fetchFollowDetails(username);
    if (data) {
      setFollowDetails(data);
    }
  });

  return (
    <div>
      <UserProfile username={username} is_followed = {followDetails().is_followed} follows = {followDetails().following} />
    </div>
  );
}