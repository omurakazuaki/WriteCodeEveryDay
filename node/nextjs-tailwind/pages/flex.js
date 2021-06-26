import Head from 'next/head'

export default function Flex() {
  return (
    <div className="flex flex-col mx-auto justify-center max-w-sm mt-8 p-16 border shadow-md">
      <h1>flex</h1>
      <div className="mt-8 flex">
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">1</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">2</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">3</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">4</div>
      </div>

      <div className="mt-8 flex flex-row">
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">1</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">2</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">3</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">4</div>
      </div>

      <div className="mt-8 flex flex-col">
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">1</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">2</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">3</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">4</div>
      </div>

      <div className="mt-8 flex flex-wrap">
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">1</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">2</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">3</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">4</div>
      </div>

      <div className="mt-8 flex">
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">1</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300 flex-grow">2</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300">3</div>
      </div>

      <div className="mt-8 flex">
        <div className="p-4 m-4 bg-gray-50 border border-gray-300 order-4">1</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300 order-3">2</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300 order-2">3</div>
        <div className="p-4 m-4 bg-gray-50 border border-gray-300 order-1">4</div>
      </div>

    </div>
  )
}
