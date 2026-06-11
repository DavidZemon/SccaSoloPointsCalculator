import { OverlayTrigger, Tooltip } from 'react-bootstrap';

export function BetterTooltip({
  label,
  text,
}: {
  label: string;
  text: string;
}) {
  return (
    <OverlayTrigger
      placement="bottom"
      overlay={<Tooltip id={`tooltip-${label}`}>{text}</Tooltip>}
    >
      <span
        style={{ cursor: 'default', marginLeft: '0.4rem', opacity: 0.7 }}
        aria-label="More information"
      >
        ⓘ
      </span>
    </OverlayTrigger>
  );
}
