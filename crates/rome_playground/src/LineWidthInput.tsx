interface Props {
  lineWidth: number;
  setLineWidth: (lineWidth: number) => void;
}

export default function LineWidthInput({ lineWidth, setLineWidth }: Props) {
  return (
    <div className="w-[300px] p-5 flex items-end">
      <div className="pr-4">
        <label
          htmlFor="lineWidth"
          className="block text-sm font-medium text-gray-700"
        >
          Line Width
        </label>
        <div className="mt-1">
          <input
            type="number"
            name="lineWidth"
            id="lineWidth"
            className="shadow-sm focus:ring-indigo-500 focus:border-indigo-500 block w-full sm:text-sm border-gray-300 rounded-md"
            value={lineWidth}
            onChange={(e) => {
              setLineWidth(parseInt(e.target.value));
            }}
          />
        </div>
      </div>
      <button
				aria-label="Set line width to 80 characters"
        onClick={() => setLineWidth(80)}
        disabled={lineWidth === 80}
        className="bg-slate-500 m-2 text-sm w-[80px] p-1 rounded text-slate-50 disabled:bg-slate-300 transition"
      >
        80
      </button>
      <button
				aria-label="Set line width to 120 characters"
        onClick={() => setLineWidth(120)}
        disabled={lineWidth === 120}
        className="bg-slate-500 m-2 text-sm w-[80px] p-1 rounded text-slate-50 disabled:bg-slate-300 transition"
      >
        120
      </button>
    </div>
  );
}
