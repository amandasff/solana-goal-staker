import Head from 'next/head';

export default function Home() {
  return (
    <div>
      <Head>
        <title>Solana Goal Staker</title>
      </Head>
      <main>
        <h1>Stake SOL for your goal</h1>
        <form>
          <input type="text" placeholder="Your goal" />
          <input type="number" placeholder="Stake amount (lamports)" />
          <button type="submit">Submit</button>
        </form>
      </main>
    </div>
  );
}
