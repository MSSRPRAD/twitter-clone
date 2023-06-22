import { useParams } from "@solidjs/router";
import UserProfile from "~/components/UserProfile";
import { createSignal, createEffect } from 'solid-js';
import TweetList from "~/components/TweetList";
import CreateTweetCard from "~/components/CreateTweetCard";
import Tweet from "~/types/Tweet";
type UserProps = {
  username: string;
};

type Data = {
  tweets: TweetDetails[];
}

type UserTweetsResponse = {
  status: String;
  results: number;
  quoted_tweets: { [key: number]: Tweet };
  tweets: Tweet[];
}

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

  const [tweets, setTweets] = createSignal([] as  Tweet[]);
  const [quote_tweets, setQuoteTweets] = createSignal({} as { [key: number]: Tweet });
  const [profileUrl, setProfileUrl] = createSignal("");
  // const test = profileUrl();
  const addUrl = (url: string) => {
    setProfileUrl(url);
  }

  createEffect(async () => {
    const follow_details = await fetch_follow_details(username);
    if (follow_details) {
      setFollowDetails(follow_details);
      console.log("follow-data:");
      console.log(follow_details);
    }
    const _resp = await fetch_tweets(username);
  });

  const setStuff = (tweets: Tweet[], quoted_tweets: {[key: number]: Tweet}) => {
    setTweets(tweets);
    setQuoteTweets(quoted_tweets);
  }

  async function fetch_tweets(username: string): Promise<UserTweetsResponse | null> {
  try {
    const response = await fetch('http://localhost:8000/twitter/' + username + '/tweets/all', {
      method: 'GET',
      credentials: "include",
    })
    if (response.ok) {
      const data = await response.json();
      console.log("data");
      console.log(data);
      setStuff(data.tweets, data.quoted_tweets);
      return data as UserTweetsResponse;
    } else {
      console.error('Error fetching tweets:', response.status);
      return null;
    }
  } catch (error) {
    console.error('Error fetching tweets:', error);
    return null;
  }
}
// Argument of type '(prev: Tweet[]) => { tweets: Tweet[]; }' is not assignable to parameter of type '(prev: Tweet[]) => Tweet[]'. 
  const addTweet = (tweet: Tweet) => {
    setTweets(prev => {
      return  [tweet, ...prev] as Tweet[];
    });
  }


  return (
    <div class="bg-gray-800 w-full">
      <UserProfile username={username} is_followed={followDetails().is_followed} follows={followDetails().following} addUrl={addUrl} no_of_followers={followDetails().no_of_followers} no_of_following={followDetails().no_of_following} />
      <div class="flex items-center">
        <CreateTweetCard quote_id={null} parent_id={null} addTweet = {addTweet} />
      </div>
      <ul class="list-none">
        <TweetList quote_tweets={quote_tweets()} tweets={tweets() as Tweet[]} profileUrl = {profileUrl()}/>
      </ul>
    </div>
  );
}