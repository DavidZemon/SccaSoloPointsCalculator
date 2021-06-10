import { EOL } from 'os';
import { Component, ComponentPropsWithoutRef } from 'react';
import { Accordion, Button, Card, Col, Row, Table } from 'react-bootstrap';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faDownload } from '@fortawesome/free-solid-svg-icons';
import {
  ClassCategoryResults,
  ClassResults,
  EventResults as EventResultsData,
} from '../models';
import { EventResultsParser, PaxService } from '../services';
import { RamDownload } from './DownloadButton';

interface EventResultsProps extends ComponentPropsWithoutRef<any> {
  paxService: PaxService;
  results?: EventResultsData;
}

interface EventResultsState {
  csvContent?: string;
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
    if (this.props.results) {
      return [
        <Row>
          <Col>
            <h2>Event Results</h2>

            <Accordion>
              <Card>
                <Card.Header key={'class'}>
                  <Accordion.Toggle
                    eventKey={'class'}
                    as={Button}
                    variant={'link'}
                  >
                    Results by Class
                  </Accordion.Toggle>
                  <Button variant={'secondary'}>
                    <FontAwesomeIcon
                      className={'clickable'}
                      icon={faDownload}
                      onClick={() => this.exportResultsByClassCsv()}
                    />
                  </Button>
                </Card.Header>

                <Accordion.Collapse eventKey={'class'}>
                  <Card.Body>
                    <Accordion>
                      {Object.entries(this.props.results).map(
                        ([classCategory, categoryResults], index) => (
                          <Card key={index}>
                            <Card.Header>
                              <Accordion.Toggle
                                eventKey={`${index}`}
                                as={Button}
                                variant={'link'}
                              >
                                {classCategory}
                              </Accordion.Toggle>
                            </Card.Header>

                            <Accordion.Collapse eventKey={`${index}`}>
                              <Card.Body>
                                {this.displayCategoryResults(categoryResults)}
                              </Card.Body>
                            </Accordion.Collapse>
                          </Card>
                        ),
                      )}
                    </Accordion>
                  </Card.Body>
                </Accordion.Collapse>
              </Card>

              <Card>
                <Card.Header key={'pax'}>
                  <Accordion.Toggle
                    eventKey={'pax'}
                    as={Button}
                    variant={'link'}
                  >
                    PAX Results
                  </Accordion.Toggle>
                  <Button variant={'secondary'} disabled>
                    <FontAwesomeIcon
                      className={'clickable'}
                      icon={faDownload}
                      onClick={() => this.exportPaxResultsCsv()}
                    />
                    &nbsp;&lt;&mdash; This button doesn't do anything yet
                  </Button>
                </Card.Header>

                <Accordion.Collapse eventKey={'pax'}>
                  <Card.Body>
                    <Table striped hover borderless>
                      <thead>
                        <tr>
                          <th>Position</th>
                          <th>Class</th>
                          <th>Car #</th>
                          <th>Name</th>
                          <th>Car</th>
                          <th>
                            {/* eslint-disable-next-line react/jsx-no-target-blank */}
                            <a
                              href={'https://www.solotime.info/pax/'}
                              target={'_blank'}
                              rel={'help'}
                            >
                              PAX Time
                            </a>
                          </th>
                        </tr>
                      </thead>

                      <tbody>
                        {Object.values(this.props.results)
                          .map((categoryResults) =>
                            Object.values(categoryResults),
                          )
                          .flat()
                          .map((classResults) => classResults.drivers)
                          .flat()
                          .sort(
                            (a, b) =>
                              (a.bestLap().time || Infinity) *
                                this.props.paxService.getMultiplierFromLongName(
                                  a.carClass,
                                ) -
                              (b.bestLap().time || Infinity) *
                                this.props.paxService.getMultiplierFromLongName(
                                  b.carClass,
                                ),
                          )
                          .map((driver, index) => (
                            <tr>
                              <td>{index + 1}</td>
                              <td>{driver.carClass}</td>
                              <td>{driver.carNumber}</td>
                              <td>{driver.name}</td>
                              <td>{driver.carDescription}</td>
                              <td>
                                {driver
                                  .bestLap()
                                  ?.toString(
                                    this.props.paxService.getMultiplierFromLongName(
                                      driver.carClass,
                                    ),
                                  )}
                              </td>
                            </tr>
                          ))}
                      </tbody>
                    </Table>
                  </Card.Body>
                </Accordion.Collapse>
              </Card>

              <Card>
                <Card.Header key={'raw'}>
                  <Accordion.Toggle
                    eventKey={'raw'}
                    as={Button}
                    variant={'link'}
                  >
                    Raw Results
                  </Accordion.Toggle>
                  <Button variant={'secondary'} disabled>
                    <FontAwesomeIcon
                      className={'clickable'}
                      icon={faDownload}
                      onClick={() => this.exportRawResultsCsv()}
                    />
                    &nbsp;&lt;&mdash; This button doesn't do anything yet
                  </Button>
                </Card.Header>

                <Accordion.Collapse eventKey={'raw'}>
                  <Card.Body>
                    <Table striped hover borderless>
                      <thead>
                        <tr>
                          <th>Position</th>
                          <th>Class</th>
                          <th>Car #</th>
                          <th>Name</th>
                          <th>Car</th>
                          <th>Raw Corrected Time</th>
                        </tr>
                      </thead>

                      <tbody>
                        {Object.values(this.props.results)
                          .map((categoryResults) =>
                            Object.values(categoryResults),
                          )
                          .flat()
                          .map((classResults) => classResults.drivers)
                          .flat()
                          .sort(
                            (a, b) =>
                              (a.bestLap().time || Infinity) -
                              (b.bestLap().time || Infinity),
                          )
                          .map((driver, index) => (
                            <tr>
                              <td>{index + 1}</td>
                              <td>{driver.carClass}</td>
                              <td>{driver.carNumber}</td>
                              <td>{driver.name}</td>
                              <td>{driver.carDescription}</td>
                              <td>{driver.bestLap()?.toString()}</td>
                            </tr>
                          ))}
                      </tbody>
                    </Table>
                  </Card.Body>
                </Accordion.Collapse>
              </Card>
            </Accordion>
          </Col>
        </Row>,
        <RamDownload
          filename={'event_results.csv'}
          content={this.state.csvContent}
          contentType={'text/csv'}
          downloadComplete={() => {
            this.setState({ csvContent: undefined });
            console.log('Download is complete');
          }}
        />,
      ];
    } else {
      return null;
    }
  }

  private displayCategoryResults(
    categoryResults: ClassCategoryResults,
  ): JSX.Element[] {
    return EventResults.convertCategoryResults(
      categoryResults,
      (classResults, index) => (
        <Table key={index} striped hover borderless>
          <thead>
            <tr>
              <th colSpan={10}>
                {classResults.carClass} (Trophies: {classResults.trophyCount})
              </th>
            </tr>
            <tr>
              {EventResultsParser.HEADER.slice(0, 6).map((header, index) => (
                <th key={index}>{header}</th>
              ))}
              <th colSpan={12}>Lap Times</th>
              <th>Fastest</th>
              <th>Difference</th>
            </tr>
          </thead>
          <tbody>{this.displayClassResults(classResults)}</tbody>
        </Table>
      ),
    );
  }

  private displayClassResults(classResults: ClassResults): JSX.Element[] {
    const bestLapInClass = classResults.getBestInClass();
    return classResults.drivers.map((driver, index) => {
      const driverBestLap = driver.bestLap();
      return (
        <tr key={index}>
          <td>{driver.trophy ? 'T' : ''}</td>
          <td>{driver.rookie ? 'R' : ''}</td>
          <td>{driver.position}</td>
          <td>{driver.carNumber}</td>
          <td>{driver.name}</td>
          <td>{driver.carDescription}</td>
          {driver.times.map((lapTime, index) => (
            <td key={index}>{lapTime.toString()}</td>
          ))}
          {new Array(12 - driver.times.length).fill(null).map((_, index) => (
            <td key={index} />
          ))}
          <td>{driverBestLap.toString()}</td>
          <td>{driver.difference(bestLapInClass)}</td>
        </tr>
      );
    });
  }

  private exportResultsByClassCsv() {
    const lines = Object.entries(this.props.results!)
      .map(([classCategory, categoryResults]) => [
        [classCategory],
        ...EventResults.exportCategoryResultsToCsv(categoryResults),
      ])
      .flat();
    this.setState({
      csvContent: lines.map((line) => `"${line.join('","')}"`).join(EOL),
    });
  }

  private exportPaxResultsCsv() {}

  private exportRawResultsCsv() {}

  private static exportCategoryResultsToCsv(
    categoryResults: ClassCategoryResults,
  ): string[][] {
    return this.convertCategoryResults(categoryResults, (classResults) => {
      const bestLapInClass = classResults.getBestInClass();
      return [
        [
          classResults.carClass,
          `Drivers: ${classResults.drivers.length}`,
          `Trophies: ${classResults.trophyCount}`,
        ],
        EventResultsParser.HEADER.slice(0, 6),
        ...classResults.drivers.map((driver) => {
          const driverBestLap = driver.bestLap();
          return [
            driver.trophy ? 'T' : '',
            driver.rookie ? 'R' : '',
            `${driver.position}`,
            `${driver.carNumber}`,
            driver.name,
            driver.carDescription,
            ...driver.times.map((t) => t.toString()),
            ...new Array(12 - driver.times.length).fill(''),
            driverBestLap.toString(),
            driver.difference(bestLapInClass),
          ];
        }),
      ];
    }).flat();
  }

  private static convertCategoryResults<T>(
    categoryResults: ClassCategoryResults,
    converter: (classResults: ClassResults, index: number) => T,
  ): T[] {
    return Object.values(categoryResults)
      .filter((classResults) => classResults.drivers.length)
      .sort((a, b) => {
        if (a.carClass < b.carClass) return -1;
        if (a.carClass > b.carClass) return 1;
        else return 0;
      })
      .map(converter);
  }
}
