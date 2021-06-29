import Link from 'next/link'

export default function Transition() {
  return (
    <div className="flex flex-col mx-auto max-w-md mt-8 p-16 border shadow-md">
      <h1>Transition</h1>
      <div className="m-8 w-32 h-32 bg-gray-50 border border-gray-300 transition-all duration-500 ease-in hover:w-64 hover:h-64"/>
      <div className="m-8 w-32 h-32 bg-gray-50 border border-gray-300 transition-all duration-500 ease-in hover:ml-32"/>
      <div className="m-8 w-32 h-32 bg-gray-50 border border-gray-300 transition-all duration-500 ease-in hover:bg-gray-300"/>
      <Link href="/"><a className="text-indigo-500 hover:underline">Home</a></Link>
    </div>
  )
}
