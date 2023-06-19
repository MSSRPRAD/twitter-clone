import { useParams } from "@solidjs/router";
import UserProfile from "~/components/UserProfile";
import { createSignal, createEffect } from 'solid-js';
import TweetCard from "~/components/TweetCard";
import CreateTweetCard from "~/components/CreateTweetCard";

type UserProps = {
  username: string;
};

type FollowDetails = {
  requesting: String;
  requested: String;
  following: boolean;
  is_followed: boolean;
  no_of_followers: number;
  no_of_following: number;
};

type TweetDetails = {
  content: string;
  created_at: string;
  likes: number;
  parent_id: number | null;
  quote_id: number | null;
  quotes: number;
  replies: number;
  retweets: number;
  tweet_id: number;
  username: string;
  views: number;
};

async function fetch_follow_details(username: string): Promise<FollowDetails | null> {
  try {
    const response = await fetch('http://localhost:8000/followdetails/' + username, {
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

async function fetch_tweets(username: string): Promise<TweetDetails | null> {
  try {
    const response = await fetch('http://localhost:8000/twitter/' + username + '/tweets/all', {
      method: 'GET',
      credentials: "include",
    })
    if (response.ok) {
      const data = await response.json();
      return data as TweetDetails;
    } else {
      console.error('Error fetching tweets:', response.status);
      return null;
    }
  } catch (error) {
    console.error('Error fetching tweets:', error);
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
    no_of_following: 0,
    no_of_followers: 0,
  });

  const [tweets, setTweets] = createSignal({
    tweets: [],
  });

  createEffect(async () => {
    const follow_details = await fetch_follow_details(username);
    if (follow_details) {
      setFollowDetails(follow_details);
      console.log("follow-data:");
      console.log(follow_details);
    }
    const tweets = await fetch_tweets(username);
    if (tweets) {
      setTweets(tweets);
      console.log("follow-data:");
      console.log(tweets);
    }
  });

  return (
    <div class="bg-gray-800 w-full">
      <UserProfile username={username} is_followed={followDetails().is_followed} follows={followDetails().following} no_of_followers={followDetails().no_of_followers} no_of_following={followDetails().no_of_following} />
      <div class="flex items-center">
        <CreateTweetCard />
      </div>
      <ul class="list-none">
        <li>
          <TweetCard tweets={tweets().tweets} username={username} />
        </li>
      </ul>
    </div>
  );
}