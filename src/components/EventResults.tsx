import { Component, ComponentPropsWithoutRef } from 'react';
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

interface EventResultsProps extends ComponentPropsWithoutRef<any> {
  pointsEngine: SccaSoloPointsEngine;
}

interface EventResultsState {
  csvContent?: string;
  exportFilename?: string;
}

export class EventResults extends Component<
  EventResultsProps,
  EventResultsState
> {
  constructor(props: Readonly<EventResultsProps>) {
    super(props);
    this.state = {};
  }

  public render() {
    if (this.props.pointsEngine) {
      return (
        <>
          <Row className={'top-buffer'}>
            <Col>
              <h2>Event Results</h2>

              <Accordion>
                {this.displayClassResults()}

                {this.displayCombinedResults(DriverGroup.PAX)}

                {this.displayCombinedResults(DriverGroup.Raw)}

                {this.displayCombinedResults(DriverGroup.Novice)}

                {this.displayCombinedResults(DriverGroup.Ladies)}
              </Accordion>
            </Col>
          </Row>

          <RamDownload
            filename={this.state.exportFilename}
            content={this.state.csvContent}
            contentType={'text/csv'}
            downloadComplete={() =>
              this.setState({
                csvContent: undefined,
                exportFilename: undefined,
              })
            }
          />
        </>
      );
    } else {
      return null;
    }
  }

  private displayClassResults(): JSX.Element {
    const header = this.props.pointsEngine.get_header_for_event_class_results();
    const classResults =
      this.props.pointsEngine.get_event_class_results_csvs() as [
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
              onClick={() =>
                this.setState({
                  exportFilename: 'event_class_results.csv',
                  csvContent: [
                    `${header}\n`,
                    ...classResults.map(
                      ([carClass, csv]) =>
                        `${this.prettyNameBuilder(carClass)}\n${csv}`,
                    ),
                  ].join(''),
                })
              }
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
                      {this.prettyNameBuilder(carClass)}
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
  }

  private displayCombinedResults(driverGroup: DriverGroup): JSX.Element {
    const csvContent =
      this.props.pointsEngine.get_event_combined_csv(driverGroup);
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
            onClick={() =>
              this.setState({
                exportFilename: `event_${DriverGroup[
                  driverGroup
                ].toLowerCase()}_results.csv`,
                csvContent,
              })
            }
          >
            <FontAwesomeIcon className={'clickable'} icon={faDownload} />
          </Button>
        </Card.Header>

        <Accordion.Collapse eventKey={DriverGroup[driverGroup]}>
          <Card.Body>
            <CsvTable
              csv={csvContent}
              keyBuilder={(driver) =>
                `${DriverGroup[driverGroup]} - ${driver[1]}`
              }
            />
          </Card.Body>
        </Accordion.Collapse>
      </Card>
    );
  }

  private prettyNameBuilder(carClass: MangledCarClass): string {
    return `${carClass.short} - ${to_display_name(
      LongCarClass[carClass.long],
    )}`;
  }
}
