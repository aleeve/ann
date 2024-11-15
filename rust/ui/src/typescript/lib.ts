import { AtpAgent } from '@atproto/api'

interface Actor {
  did: string;
  texts: Array<string>;
}

interface Follow {
  did: string;
  follows: Array<string>;
}

const agent = new AtpAgent({
  service: 'https://bsky.social'
})


async function getActor(did: string): Promise<Actor> {
  var texts = await agent.com.atproto.repo.listRecords(
    {
      repo: did,
      collection: "app.bsky.feed.post",
    },
    {
      headers: {
        "Accept-Language": 'en',
      },
    },
  );

  console.log(texts)
  var texts = texts.data.records.map((rec) => rec.value.text);
  return { did, texts }
}

async function getFollows(did: string): Promise<Follow> {
  let response = await agent.com.atproto.repo.listRecords(
    {
      repo: did,
      collection: "app.bsky.graph.follow",
    },
    {
      headers: {
        "Accept-Language": 'en',
      },
    },
  )
  console.log(response)

  let follows = response.data.records.map((rec) => rec.value.subject);
  return { did, follows }
}

// Expose the function to the global scope
(globalThis as any).getActor = getActor;
(globalThis as any).getFollows = getFollows;
