import { useParams } from "@solidjs/router";
import UserProfile from "~/components/UserProfile";
import { createSignal, createEffect } from 'solid-js';
import TweetList from "~/components/TweetList";
import CreateTweetCard from "~/components/CreateTweetCard";
import Tweet from "~/types/Tweet";


type UserTweetsResponse = {
  status: String;
  results: number;
  parent_chain: Tweet[];
  tweet: Tweet[];
  quoted_tweets: { [key: number]: Tweet  }
  replies: Tweet[];
  profile_pics: { [key: number]: String };
}

const foo = () => {
  return;
}

export default function ViewTweet() {
  const params = useParams(); // 👈 Get the dynamic route parameters
  console.log(params);
  const tweet_id = params.tweet_id;
  const [parentChain, setParentChain] = createSignal([] as  Tweet[]);
  const [replies, setReplies] = createSignal([] as Tweet[]);
  const [profilePics, setProfilePics] = createSignal( {} as { [key: number]: String } );
  const [quoteTweets, setQuoteTweets] = createSignal(  {} as { [key: number]: Tweet } );
  const [tweet, setTweet] = createSignal( [] as Tweet[] );

  createEffect(async () => {
    const _resp = await fetch_tweets(tweet_id);
  });

  const setStuff = (quoteTweets: {[key: number]: Tweet}, replies: Tweet[], parentChain: Tweet[], profile_pics: { [key: number]: String }, tweet: Tweet) => {
    setQuoteTweets(quoteTweets);
    setReplies(replies);
    setParentChain(parentChain);
    setProfilePics(profile_pics);
    const arr: Tweet[] = [];
    arr[0] = tweet;
    setTweet(arr);
  }

  async function fetch_tweets(tweet_id: string): Promise<UserTweetsResponse | null> {
  try {
    const response = await fetch('http://localhost:8000/twitter/'+tweet_id+'/tweetchain', {
      method: 'GET',
      credentials: "include",
    })
    if (response.ok) {
      const data = await response.json();
      console.log("data");
      console.log(data);
      setStuff(data.quoted_tweets,data.replies, data.parent_chain, data.profile_pics, data.tweet);
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
  return (
    <div class="bg-gray-800 w-full">
      <div>
        <CreateTweetCard quote_id={null} parent_id={null} addTweet = {foo} />
      </div>
      <ul class="list-none w-full flex justify-center items-center">
        <TweetList quote_tweets={quoteTweets()} tweets={parentChain()} profileUrl = {null} profile_pics = { profilePics() }/>
      </ul>
      /*
      <ul class="list-none w-full">
        <TweetList quote_tweets={quoteTweets()} tweets={tweet()} profileUrl = {null} profile_pics = { profilePics() }/>
      </ul>
      */
      <ul class="list-none w-5/6 flex justify-center items-center">
        <TweetList quote_tweets={quoteTweets()} tweets={replies()} profileUrl = {null} profile_pics = { profilePics() }/>
      </ul>
    </div>
  );
}
