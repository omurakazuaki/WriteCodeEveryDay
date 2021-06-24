import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <>
      <div className="mt-16 max-w-md mx-auto bg-white rounded-xl shadow-md overflow-hidden md:max-w-2xl">
        <div className="md:flex">
          <div className="md:flex-shrink-0">
            <img className="App-logo h-48 w-full object-cover md:h-full md:w-48" src={logo} alt="logo" />
          </div>
          <div className="p-8">
            <div className="uppercase tracking-wide text-sm text-indigo-500 font-semibold">Hello React with tailwind</div>
            <a href="https://tailwindcss.com/docs/guides/create-react-app" target="_blank" rel="noreferrer" className="block mt-1 text-lg leading-tight font-medium text-black hover:underline">Install Tailwind CSS with Create React App</a>
            <p className="mt-2 text-gray-500">Edit <code>src/App.js</code> and save to reload.</p>
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
