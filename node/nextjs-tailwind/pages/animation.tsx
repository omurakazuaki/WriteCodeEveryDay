import Link from 'next/link'

export default function Flex() {
  return (
    <div className="flex flex-col mx-auto justify-center max-w-sm mt-8 p-16 border shadow-md">
      <h1>Animation</h1>
      <div className="m-16 p-8 w-32 h-32 bg-gray-50 border border-gray-300 animate-spin"/>
      <div className="m-16 p-8 w-32 h-32 bg-gray-50 border border-gray-300 animate-ping"/>
      <div className="m-16 p-8 w-32 h-32 bg-gray-50 border border-gray-300 animate-pulse"/>
      <div className="m-16 p-8 w-32 h-32 bg-gray-50 border border-gray-300 animate-bounce"/>
      <Link href="/"><a className="text-indigo-500 hover:underline">Home</a></Link>
    </div>
  )
}
