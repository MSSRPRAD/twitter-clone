type Tweet = {
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