// invalid
<div onMouseOver={() => {}} />;
<div onMouseOut={() => {}} />;
<div onMouseOver={() => {}} onFocus={undefined} />;
<div onMouseOut={() => {}} onBlur={undefined} />;
<div onMouseOver={() => {}} onFocus={null} />;
<div onMouseOut={() => {}} onBlur={null} />;
<div onMouseOver={() => {}}></div>;
<div onMouseOut={() => {}}></div>;
<div {...spread} onMouseOut={() => {}}></div>;
<div {...spread} onMouseOver={() => {}}></div>;

// valid
<div />;
<div onFocus={() => {}} />;
<div onMouseOver={() => {}} onFocus={() => {}} />;
<div onMouseOver={() => {}} {...props} />;
<div onMouseOver={() => {}} onFocus={() => {}} {...props} />;
<div onMouseOver={handleMouseOver} onFocus={handleFocus} />;
<div onMouseOver={handleMouseOver} onFocus={handleFocus} {...props} />;
<div onBlur={() => {}} />;
<div onMouseOut={() => {}} onBlur={() => {}} />;
<div onMouseOut={() => {}} {...props} />;
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

