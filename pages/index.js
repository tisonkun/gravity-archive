import Head from 'next/head'

export default function Page() {
  return (
    <div className="container">
      <Head>
        <title>Gravity</title>
      </Head>

      <main>
          We provides APIs under these paths now:
          <ol>
              <li>/api/contributors</li>
          </ol>
      </main>
    </div>
  )
}
