import { JSX, useCallback, useState } from 'react';
import { Accordion, Button, Card, Col, Row } from 'react-bootstrap';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDownload } from '@fortawesome/free-solid-svg-icons';
import {
  CarClass,
  ClassCategory,
  DriverGroup,
  LongCarClass,
  SccaSoloPointsEngine,
  ShortCarClass,
  to_display_name,
} from 'scca_solo_points_engine/scca_solo_points_engine';
import { RamDownload } from './DownloadButton';
import { CsvTable } from './CsvTable';

type MangledCarClass = Omit<CarClass, 'short' | 'long' | 'category'> & {
  short: keyof typeof ShortCarClass;
  long: keyof typeof LongCarClass;
  category: keyof typeof ClassCategory;
};

interface EventResultsProps {
  pointsEngine: SccaSoloPointsEngine;
}

function prettyNameBuilder(carClass: MangledCarClass): string {
  return `${carClass.short} - ${to_display_name(LongCarClass[carClass.long])}`;
}

export function EventResults({
  pointsEngine,
}: EventResultsProps): JSX.Element | null {
  const [csvContent, setCsvContent] = useState<string | undefined>();
  const [exportFilename, setExportFilename] = useState<string | undefined>();

  const displayClassResults = useCallback(() => {
    const header = pointsEngine.get_header_for_event_class_results();
    const classResults = pointsEngine.get_event_class_results_csvs() as [
      MangledCarClass,
      string,
    ][];
    return (
      <Card>
        <Card.Header key={'class'}>
          <Accordion.Toggle eventKey={'class'} as={Button} variant={'link'}>
            Results by Class
          </Accordion.Toggle>

          <Button variant={'secondary'}>
            <FontAwesomeIcon
              className={'clickable'}
              icon={faDownload}
              onClick={() => {
                setExportFilename('event_class_results.csv');
                setCsvContent(
                  [
                    `${header}\n`,
                    ...classResults.map(
                      ([carClass, csv]) =>
                        `${prettyNameBuilder(carClass)}\n${csv}`,
                    ),
                  ].join(''),
                );
              }}
            />
          </Button>
        </Card.Header>

        <Accordion.Collapse eventKey={'class'}>
          <Card.Body>
            <Accordion>
              {classResults.map(([carClass, csv]) => (
                <Card key={`class-table-${carClass.short}`}>
                  <Card.Header>
                    <Accordion.Toggle
                      eventKey={`class-table-${carClass.short}`}
                      as={Button}
                      variant={'link'}
                    >
                      {prettyNameBuilder(carClass)}
                    </Accordion.Toggle>
                  </Card.Header>

                  <Accordion.Collapse
                    eventKey={`class-table-${carClass.short}`}
                  >
                    <Card.Body>
                      <CsvTable
                        csv={`${header}\n${csv}`}
                        keyBuilder={(driver) =>
                          `${carClass.short} - ${driver[1]}`
                        }
                      />
                    </Card.Body>
                  </Accordion.Collapse>
                </Card>
              ))}
            </Accordion>
          </Card.Body>
        </Accordion.Collapse>
      </Card>
    );
  }, [pointsEngine]);
  const displayCombinedResults = useCallback(
    (driverGroup: DriverGroup) => {
      const csvContent = pointsEngine.get_event_combined_csv(driverGroup);
      return (
        <Card>
          <Card.Header key={driverGroup}>
            <Accordion.Toggle
              eventKey={DriverGroup[driverGroup]}
              as={Button}
              variant={'link'}
            >
              {DriverGroup[driverGroup]} Results
            </Accordion.Toggle>

            <Button
              variant={'secondary'}
              onClick={() => {
                setExportFilename(
                  `event_${DriverGroup[driverGroup].toLowerCase()}_results.csv`,
                );
                setCsvContent(csvContent);
              }}
            >
              <FontAwesomeIcon className={'clickable'} icon={faDownload} />
            </Button>
          </Card.Header>

          <Accordion.Collapse eventKey={DriverGroup[driverGroup]}>
            <Card.Body>
              <CsvTable
                csv={csvContent}
                keyBuilder={(driver) =>
                  `${DriverGroup[driverGroup]} - ${driver[2]}`
                }
              />
            </Card.Body>
          </Accordion.Collapse>
        </Card>
      );
    },
    [pointsEngine],
  );

  if (pointsEngine) {
    return (
      <>
        <Row className={'top-buffer'}>
          <Col>
            <h2>Event Results</h2>

            <Accordion>
              {displayClassResults()}

              {displayCombinedResults(DriverGroup.PAX)}

              {displayCombinedResults(DriverGroup.Raw)}

              {displayCombinedResults(DriverGroup.Novice)}

              {displayCombinedResults(DriverGroup.Ladies)}
            </Accordion>
          </Col>
        </Row>

        <RamDownload
          filename={exportFilename}
          content={csvContent}
          contentType={'text/csv'}
          downloadComplete={() => {
            setCsvContent(undefined);
            setExportFilename(undefined);
          }}
        />
      </>
    );
  } else {
    return null;
  }
}
