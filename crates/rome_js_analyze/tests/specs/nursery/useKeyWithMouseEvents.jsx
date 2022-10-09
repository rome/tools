// invalid
<div onMouseOver={() => {}} />;
<div onMouseOut={() => {}} />;
<div onMouseOver={() => {}} onFocus={undefined} />;
<div onMouseOut={() => {}} onBlur={undefined} />;
<div onMouseOver={() => {}} onFocus={null} />;
<div onMouseOut={() => {}} onBlur={null} />;
<div onMouseOver={() => {}} {...props} />;
<div onMouseOut={() => {}} {...props} />;
<div onMouseOver={() => {}}></div>;
<div onMouseOut={() => {}}></div>;

// valid
<div onMouseOver={() => {}} onFocus={() => {}} />;
<div onMouseOver={() => {}} onFocus={() => {}} {...props} />;
<div onMouseOver={handleMouseOver} onFocus={handleFocus} />;
<div onMouseOver={handleMouseOver} onFocus={handleFocus} {...props} />;
<div />;
<div onBlur={() => {}} />;
<div onFocus={() => {}} />;
<div onMouseOut={() => {}} onBlur={() => {}} />;
<div onMouseOut={() => {}} onBlur={() => {}} {...props} />;
<div onMouseOut={handleMouseOut} onBlur={handleOnBlur} />;
<div onMouseOut={handleMouseOut} onBlur={handleOnBlur} {...props} />;
<MyComponent />;
<MyComponent onMouseOver={() => {}} />;
<MyComponent onMouseOut={() => {}} />;
<MyComponent onBlur={() => {}} />;
<MyComponent onFocus={() => {}} />;
<MyComponent onMouseOver={() => {}} {...props} />;
<MyComponent onMouseOut={() => {}} {...props} />;
<MyComponent onBlur={() => {}} {...props} />;
<MyComponent onFocus={() => {}} {...props} />;

