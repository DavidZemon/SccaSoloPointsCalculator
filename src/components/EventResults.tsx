import { Component, ComponentPropsWithoutRef } from 'react';
import { Accordion, Button, Card, Col, Row, Table } from 'react-bootstrap';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDownload } from '@fortawesome/free-solid-svg-icons';
import { parse } from 'csv-parse/lib/sync';
import {
  ClassResultsBuilder,
  CombinedResultsBuilder,
  DriverGroup,
  DriverId,
  EventResults as EventResultsData,
  LongCarClass,
  to_display_name,
} from 'rusty/rusty';
import { RamDownload } from './DownloadButton';
import { MangledCarClass } from '../models';

interface EventResultsProps extends ComponentPropsWithoutRef<any> {
  results?: EventResultsData;
  ladiesIds?: DriverId[]; // IDs of Ladies drivers from championship results
}

interface EventResultsState {
  csvContent?: string;
  exportFilename?: string;
}

export class EventResults extends Component<
  EventResultsProps,
  EventResultsState
> {
  private comboResultsBldr?: CombinedResultsBuilder = undefined;
  private classResultsBldr?: ClassResultsBuilder = undefined;

  constructor(props: Readonly<EventResultsProps>) {
    super(props);
    this.state = {};
  }

  componentDidMount() {
    this.comboResultsBldr = new CombinedResultsBuilder();
    this.classResultsBldr = new ClassResultsBuilder();
  }

  public render() {
    if (this.props.results) {
      return (
        <>
          <Row className={'top-buffer'}>
            <Col>
              <h2>Event Results</h2>

              <Accordion>
                {this.classResultsBldr && this.displayClassResults()}

                {this.comboResultsBldr &&
                  this.displayCombinedResults(DriverGroup.PAX)}

                {this.comboResultsBldr &&
                  this.displayCombinedResults(DriverGroup.Raw)}

                {this.comboResultsBldr &&
                  this.displayCombinedResults(DriverGroup.Novice)}

                {this.comboResultsBldr &&
                  (this.props.ladiesIds
                    ? this.displayCombinedResults(DriverGroup.Ladies)
                    : null)}
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
    const header = this.classResultsBldr!.get_header();
    const classResults = this.classResultsBldr!.to_csvs(
      this.props.results!,
    ) as [MangledCarClass, string][];
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
                      {this.displayTable(
                        `${header}\n${csv}`,
                        (driver) => `${carClass.short} - ${driver[1]}`,
                      )}
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
    const csvContent = this.comboResultsBldr!.to_combined_csv(
      this.props.results!,
      driverGroup,
    );
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
            {this.displayTable(
              csvContent,
              (driver) => `${DriverGroup[driverGroup]} - ${driver[1]}`,
            )}
          </Card.Body>
        </Accordion.Collapse>
      </Card>
    );
  }

  private displayTable(
    csvContent: string,
    keyBuilder: (row: string[]) => string,
  ): JSX.Element {
    const lines: string[][] = parse(csvContent, { columns: false });
    const [header, ...drivers] = lines;
    return (
      <Table striped hover borderless>
        <thead>
          <tr>
            {header.map((h) => (
              <th key={h}>{h}</th>
            ))}
          </tr>
        </thead>

        <tbody>
          {drivers.map((row) => {
            const rowKey = keyBuilder(row);
            return (
              <tr key={rowKey}>
                {row.map((column, i) => (
                  <td key={`${rowKey} - ${header[i]}`}>{column}</td>
                ))}
              </tr>
            );
          })}
        </tbody>
      </Table>
    );
  }

  private prettyNameBuilder(carClass: MangledCarClass): string {
    return `${carClass.short} - ${to_display_name(
      LongCarClass[carClass.long],
    )}`;
  }
}
