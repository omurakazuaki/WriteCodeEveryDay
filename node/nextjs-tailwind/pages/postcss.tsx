import Link from 'next/link'

export default function PostCSS() {
  return (
    <div className="flex flex-col mx-auto justify-center max-w-sm mt-8 p-16 border shadow-md">
      <h1>PostCSS</h1>
      <div className="my-8">
        <button className="btn">Default</button>
      </div>
      <div className="my-8">
        <button className="btn btn-primary">Primary</button>
      </div>
      <Link href="/"><a className="text-indigo-500 hover:underline">Home</a></Link>
    </div>
  )
}
