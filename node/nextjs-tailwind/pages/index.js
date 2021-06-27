import Head from 'next/head'
import Link from 'next/link'

export default function Home() {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen py-2 bg-blue-50">
      <Head>
        <title>Create Next App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className="flex flex-col items-center justify-center w-full flex-1 px-20 text-center">
        <h1 className="text-6xl font-bold">
          Welcome to{' '}
          <a className="text-blue-600" href="https://nextjs.org">
            Next.js!
          </a>
        </h1>

        <p className="mt-3 text-2xl">
          Get started by editing{' '}
          <code className="p-3 font-mono text-lg bg-gray-100 rounded-md">
            pages/index.js
          </code>
        </p>

        <div className="flex flex-wrap items-center justify-around max-w-4xl mt-6 sm:w-full">

          <a
            href="https://tailwindcss.com/docs/guides/nextjs"
            className="p-6 mt-6 text-left border w-96 rounded-xl hover:text-blue-600 focus:text-blue-600 bg-white shadow-md"
          >
            <h3 className="text-2xl font-bold">Setup &rarr;</h3>
            <p className="mt-4 text-xl">
              How to Setup Next.js with TailWind.
            </p>
          </a>

          <Link href="/flex">
            <a
              className="p-6 mt-6 text-left border w-96 rounded-xl hover:text-blue-600 focus:text-blue-600 bg-white shadow-md"
            >
              <h3 className="text-2xl font-bold">Flex &rarr;</h3>
              <p className="mt-4 text-xl">
                Learn Flex with TailWind
              </p>
            </a>
          </Link>

          <Link href="/transition">
            <a
              className="p-6 mt-6 text-left border w-96 rounded-xl hover:text-blue-600 focus:text-blue-600 bg-white shadow-md"
            >
              <h3 className="text-2xl font-bold">Transition &rarr;</h3>
              <p className="mt-4 text-xl">
                Learn transition with TailWind
              </p>
            </a>
          </Link>

          <Link href="/animation">
            <a
              className="p-6 mt-6 text-left border w-96 rounded-xl hover:text-blue-600 focus:text-blue-600 bg-white shadow-md"
            >
              <h3 className="text-2xl font-bold">Animation &rarr;</h3>
              <p className="mt-4 text-xl">
                Learn Animation with TailWind
              </p>
            </a>
          </Link>

        </div>
      </main>

      <footer className="flex items-center justify-center w-full h-24 border-t bg-white shadow-sm">
        <a
          className="flex items-center justify-center"
          href="https://vercel.com?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          Powered by{' '}
          <img src="/vercel.svg" alt="Vercel Logo" className="h-4 ml-2" />
        </a>
      </footer>
    </div>
  )
}
