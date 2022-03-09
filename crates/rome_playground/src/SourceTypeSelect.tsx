interface Props {
  setIsTypeScript: (b: boolean) => void;
  isTypeScript: boolean;
  setIsJsx: (b: boolean) => void;
  isJsx: boolean;
}

export default function SourceTypeSelect({
  setIsTypeScript,
  isTypeScript,
  setIsJsx,
  isJsx,
}: Props) {
  return (
    <div className="pl-6 pb-5">
      <fieldset className="space-y-5">
        <legend className="sr-only">File Type</legend>
        <div className="relative flex items-start">
          <div className="flex items-center h-5">
            <input
              id="typescript"
              aria-describedby="typescript-description"
              name="typescript"
              type="checkbox"
              checked={isTypeScript}
              onChange={(e) => setIsTypeScript(e.target.checked)}
              className="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300 rounded"
            />
          </div>
          <div className="ml-3 text-sm">
            <label htmlFor="typescript" className="font-medium text-gray-700">
              TypeScript
            </label>
            <span id="typescript-description" className="text-gray-500">
              <span className="sr-only">TypeScript</span>
            </span>
          </div>
        </div>
        <div className="relative flex items-start">
          <div className="flex items-center h-5">
            <input
              id="jsx"
              aria-describedby="jsx-description"
              name="jsx"
              type="checkbox"
              checked={isJsx}
              onChange={(e) => setIsJsx(e.target.checked)}
              className="focus:ring-indigo-500 h-4 w-4 text-indigo-600 border-gray-300 rounded"
            />
          </div>
          <div className="ml-3 text-sm">
            <label htmlFor="jsx" className="font-medium text-gray-700">
              JSX
            </label>
            <span id="jsx-description" className="text-gray-500">
              <span className="sr-only">JSX</span>
            </span>
          </div>
        </div>
      </fieldset>
    </div>
  );
}
