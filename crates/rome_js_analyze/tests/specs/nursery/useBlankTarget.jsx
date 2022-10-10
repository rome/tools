<>
    {/* invalid */}
    <a href='http://external.link' target='_blank'>child</a>
    <a href='http://external.link' target='_BLank'>child</a>
    <a href={dynamicLink} target='_blank'>child</a>
    <a href="http://external.link" target="_blank">child</a>
    <a href="http://external.link" target="_blank" rel="noopener">child</a>
    {/*  valid  */}
    <p href='http://external.link' target='_blank'>child</p>
    <a href='http://external.link' rel='noreferrer' target='_blank'>child</a>
    <a href='http://external.link' rel='noopener noreferrer' target='_blank'>child</a>
    <a href={dynamicLink} rel='noreferrer' target='_blank'>child</a>
    <a rel="noreferrer" target="_blank">child</a>
    <a rel="noreferrer" target="_BLank">child</a>
</>