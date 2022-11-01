<>
    {/* invalid */}
    <a />
    <a href/>
    <a href={null}/>
    <a href={undefined}/>
    <a href="specs/a11y/useValidAnchor#"/>
    <a href={"#"}/>
    <a href={`#`}/>
    <a href="javascript:void(0)"/>
    <a href={"javascript:void(0)"}/>
    <a href={`javascript:void(0)`}/>
    <a onClick={}/>
    <a onClick={} href={}/>
    <a href={<span><span className="token string">javascript:void(0)</span></span>}/>
    {/*  valid  */}
    <a href={`https://www.javascript.com`}/>
    <a href={somewhere}/>
    <a {...spread }/>
</>